pub mod models;
pub mod routes;

use axum::{routing::{delete, get, post, put}, Router};
use std::path::PathBuf;

use self::routes::{
    AppState, create_directory, delete_directory, delete_file, health, list_files,
    move_directory, rename_directory, replace_file, save_single_file, upload_files_batch,
    upload_files_batch_no_replace,
};

pub fn create_router(base_dir: PathBuf) -> Router {
    let state = AppState { base_dir };

    Router::new()
        .route("/health", get(health))
        .route("/directories", post(create_directory))
        .route("/directories/{dir_name}", delete(delete_directory))
        .route("/directories/{dir_name}/rename", put(rename_directory))
        .route("/directories/{dir_name}/move", post(move_directory))
        .route("/files", get(list_files))
        .route("/files", post(save_single_file))
        .route("/files/batch", post(upload_files_batch))
        .route("/files/batch/no-replace", post(upload_files_batch_no_replace))
        .route("/files/replace", post(replace_file))
        .route("/files/{file_name}", delete(delete_file))
        .with_state(state)
}
