use std::{fs::File, io::Write, path::PathBuf};

fn main() {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let graphql_dir = project_dir.parent().unwrap().join("graphql");
    let schema = graphql::new_schema().sdl();
    let schema_path = graphql_dir.join("schema").join("schema.graphql");

    if schema_path.exists() {
        std::fs::remove_file(schema_path.clone()).unwrap();
    }
    let mut f = File::create(schema_path).unwrap();
    f.write_all(schema.as_bytes()).unwrap();
    f.flush().unwrap();
}
