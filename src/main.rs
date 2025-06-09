use std::sync::{Arc, Mutex};
use sysinfo::{System, MINIMUM_CPU_UPDATE_INTERVAL};
use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};

#[tokio::main]
async fn main() {
    let app_state = AppState::default();
    let app = Router::new()
        .route("/", get(root_get))
        .route("/api/cpus", get(cpus_get)) 
        .with_state(app_state.clone());

    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu_usage();
            let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            {
                let mut cpus = app_state.cpus.lock().unwrap();
                *cpus = v;
            }

            std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
        }
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

#[derive(Default,Clone)]
struct AppState {
    cpus: Arc<Mutex<Vec<f32>>>,
}

async fn root_get() -> &'static str {
    "hello world!"
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse{
    let v = state.cpus.lock().unwrap().clone();
    Json(v)
}
