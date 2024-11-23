use std::sync::Arc;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::{engine::remote::ws::Client, opt::auth::Root, Surreal};
use warp::Filter;

/// Function to establish and return a SurrealDB connection
async fn connect_to_db() -> surrealdb::Result<Arc<Surreal<Client>>> {
    // Create a new SurrealDB client
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Authenticate with root credentials
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select the namespace and database
    db.use_ns("pennywise").use_db("budget").await?;

    // Wrap the connection in an Arc for shared use
    Ok(Arc::new(db))
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Initialize tracing for debugging
    tracing_subscriber::fmt().init();

    let db = connect_to_db().await?;

    // Share database across routes
    let db = Arc::new(db);

    // Define a basic route
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    // Start the server
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
