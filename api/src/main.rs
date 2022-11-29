use actix_web::{guard, middleware::Logger, web, App, HttpServer};
use api::{index, playground};
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let _ = env_logger::try_init();

    if let Ok(p) = dotenv::dotenv() {
        println!("Loading environment from {}", p.display());
    }
    let address = "127.0.0.1:8080";

    let _ = tracing_subscriber::fmt()
        .with_thread_names(true)
        .with_thread_ids(true)
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .finish()
        .try_init()
        .ok();

    let app_data = api::AppData::new();
    tracing::info!("Staring server on address: {}", address);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(graphql::new_schema()))
            .app_data(web::Data::new(app_data.clone()))
            .wrap(Logger::default())
            .service(
                web::resource("/graphql").route(
                    web::route()
                        .guard(
                            guard::Any(guard::Get())
                                .or(guard::Post())
                                .or(guard::Options()),
                        )
                        .to(index),
                ),
            )
            .service(
                web::resource("/playground").route(
                    web::route()
                        .guard(guard::Any(guard::Get()).or(guard::Options()))
                        .to(playground),
                ),
            )
    })
    .bind(address)?
    .run()
    .await
}
