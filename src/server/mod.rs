mod handlers;
mod markup;

use axum::{
    Router,
    routing::{get, post},
};

use crate::server::handlers::accept_drawing;

pub async fn start_server() {
    let app = Router::new()
        .route("/", get(markup::index))
        .route("/drawings", post(accept_drawing));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
