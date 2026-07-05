use std::{fmt::Display, time::Duration};

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderValue, Method, header::CONTENT_TYPE},
    response::Html,
    routing::{get, post},
};

use serde::Deserialize;
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    time::sleep,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);

    tokio::spawn(consume(rx));

    let app = Router::new()
        .route("/", get(home))
        .route("/comments", post(handle_comments_post_form))
        .layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_methods([Method::POST])
                    .allow_origin(HeaderValue::from_static("http://127.0.0.1:8000"))
                    .allow_headers([CONTENT_TYPE]),
            ),
        )
        .with_state(AppState { tx: tx });

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    tx: Sender<Comment>,
}

#[derive(Deserialize, Debug)]
struct Comment {
    name: String,
    body: String,
    slug: String,
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} - {}", self.name, self.body, self.slug)
    }
}

async fn home() -> Html<String> {
    Html(r#"<strong>Hello, world!</strong>"#.to_string())
}

async fn handle_comments_post_form(State(state): State<AppState>, Json(comment): Json<Comment>) {
    let tx = state.tx;
    tx.send(comment).await.unwrap()
}

async fn consume(mut rx: Receiver<Comment>) {
    let mut i = 0;

    while let Some(comment) = rx.recv().await {
        println!("Received: {}", comment);
        println!("Sleeping for {i} seconds");
        sleep(Duration::from_secs(i)).await;
        i += 1;
    }
}
