use include_dir::{include_dir, Dir};
use sqlx_pg_migrate::migrate;
use tonic::transport::Server;
mod model;

pub mod grpc_protos {
    tonic::include_proto!("rust_app");
}
use sqlx::postgres::PgPoolOptions;
#[path = "user/user_controller.rs"]
mod user_controller;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const DB_URL: &str = "postgresql://postgres:pass123@localhost:5432/rust_database";
    const MIGRATIONS: Dir = include_dir!("migrations");
    migrate(&DB_URL, &MIGRATIONS).await?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&DB_URL)
        .await?;

    let addr = "127.0.0.1:50051".parse().unwrap();
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(user_controller::new_grpc_service(pool))
        .serve(addr)
        .await?;
    Ok(())
}
