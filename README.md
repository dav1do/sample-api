
# Sample API

## Local 

- copy `.env.sample` to `.env`
- `cargo run` will host on 127.0.0.1:8080

`127.0.0.1:8080/playground` -> get the playground
`127.0.0.1:8080/graphql` -> execute requests against the api

## Sample Requests

```
mutation signup {
  signup(input:{email:"david@test.com", password: "qwerty", name: "david"}) {
    name
    email
  }
}

mutation login {
  login(input: {email: "david@test.com", password:"qwerty"}) {
    token
  }
}

query favs($token: String) {
  getUser(input: {token:$token}) {
    name
    email
    favoriteCities {
      name
      country
    }
  }
}

mutation addCity($token: String!) {
  addFavoriteCity(
    input: { token: $token, name: "Minneapolis", country: "USA" }
  ) {
    city {
      name
      country
    }
  }
}

mutation removeCity($token: String!) {
  removeFavoriteCity(input: { token: $token, name: "Minneapolis", country: "USA" }) {
    success
  }
}

```

## Limitations
- The database is in memory and will disappear each restart
- Tokens are not handled as JWTs but instead passed in as post body parameters
- Nothing is configurable (e.g. port)
- Data models are trivial (no created_at/updated_at timestamps)
- No tests... decided to fix city name collison bug instead of adding them
- No metrics, logs need

```rust
use actix_web::{
    test::{self, init_service, TestRequest},
    web, App,
};

// could test with integration approach using something like what is in main.rs plus 
let app = init_service(
    App::new()
        .app_data(data.clone())
        .app_data(schema.clone())
        .wrap(actix_web::middleware::Logger::default())
        .service(routes),
)
.await;

let resp = test::call_and_read_body_json(&app, req.to_request()).await;
resp
```

## Tasks
Should be able to accomplish the following:

    * Create a user account with the following:
    
        * Name
        
        * Email
        
        * Password
        
    * A user should be able to authenticate with an email and password
        
    * An authenticated user should be able to add and remove their favorite cities where a city consists of the following:
    
        * City name
        
        * Country
        
    * An authenticated user should be able to retrieve a list of their favorite cities
