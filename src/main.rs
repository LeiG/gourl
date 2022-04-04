use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    AddExtensionLayer, Json, Router,
};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    // Read DATABASE_URL from .env file.
    dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt::init();

    // set up connection pool
    let manager = PostgresConnectionManager::new

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `GET /go/:short_url` goes to `go_short_url`
        .route("/go/:short_url", get(go_short_url))
        // `POST /index` goest to index_short_url
        .route("/index", post(index_short_url));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn go_short_url(Path(short_url): Path<String>) -> Redirect {
    // short_url lookup
    tracing::info!("Goto short_url {}", short_url);

    Redirect::to("http://www.google.com".parse().unwrap())
}

async fn index_short_url(Json(payload): Json<IndexShortUrl>) -> impl IntoResponse {
    let short_url = ShortUrl {
        short_url: payload.short_url.unwrap_or("test".to_string()),
        url: payload.url,
    };

    (StatusCode::CREATED, Json(short_url))
}

#[derive(Deserialize)]
struct IndexShortUrl {
    url: String,
    short_url: Option<String>,
}

#[derive(Serialize)]
struct ShortUrl {
    short_url: String,
    url: String,
}
