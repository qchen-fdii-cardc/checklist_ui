use std::net::SocketAddr;

use axum::{
    extract::State,
    response::Html,
    routing::get,
    Router,
};
use clap::Parser;
use tower_http::trace::TraceLayer;

#[derive(Parser, Debug)]
#[command(author, version, about = "Checklist UI")]
struct Cli {
    #[arg(long, default_value = "http://127.0.0.1:3000")]
    api_base: String,
    #[arg(long, default_value = "0.0.0.0:8090")]
    bind: String,
}

#[derive(Clone)]
struct AppState {
    api_base: String,
}

fn ui_html(api_base: &str) -> String {
    let safe_api_base = api_base.replace('\\', "\\\\").replace('"', "\\\"");
    include_str!("../static/index.html")
        .replace("__API_BASE__", &safe_api_base)
}

async fn index(State(state): State<AppState>) -> Html<String> {
    Html(ui_html(&state.api_base))
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let state = AppState {
        api_base: cli.api_base,
    };

    let app = Router::new()
        .route("/", get(index))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr: SocketAddr = cli.bind.parse().expect("invalid bind address");
    println!("Checklist UI listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind UI port");
    axum::serve(listener, app).await.expect("UI server failed");
}
