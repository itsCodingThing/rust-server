mod router;
mod utils;

extern crate dotenv;
use dotenv::dotenv;
use std::{env, process};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| {
        println!("error loading .env file");
        process::exit(1);
    });

    match tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await {
        Ok(listener) => {
            println!("server is running port: {port} 🚀");
            axum::serve(listener, router::routes())
                .await
                .unwrap_or_else(|_| println!("unable to start server !!!"));
        }
        Err(_) => {
            println!("unable to start server !!!")
        }
    };
}
