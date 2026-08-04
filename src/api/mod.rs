pub mod models;
pub mod routes;

use axum::{routing::{delete, get, post}, Router};
use std::path::PathBuf;

use self::routes::{AppState, create_directory, delete_file, health, list_files, replace_file, upload_file};

pub fn create_router(base_dir: PathBuf) -> Router {
    let state = AppState { base_dir };

    Router::new()
        .route("/health", get(health))
        .route("/directories", post(create_directory))
        .route("/files", get(list_files))
        .route("/files", post(upload_file))
        .route("/files/replace", post(replace_file))
        .route("/files/{file_name}", delete(delete_file))
        .with_state(state)
}
