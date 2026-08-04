use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::path::PathBuf;

use crate::{
    api::models::{
        BatchUploadFileRequest, CreateDirRequest, FileResponse, ListFilesQuery,
        MoveDirectoryRequest, RenameDirectoryRequest, UploadFileRequest,
        DirectoryResponse,
    },
    errors::AppError,
    fs_ops,
};

#[derive(Clone)]
pub struct AppState {
    pub base_dir: PathBuf,
}

pub async fn health() -> &'static str {
    "ok"
}

pub async fn create_directory(
    State(state): State<AppState>,
    Json(payload): Json<CreateDirRequest>,
) -> Result<Json<FileResponse>, ApiError> {
    let dir_name = payload.name.clone();
    fs_ops::create_new_dir(&state.base_dir, &dir_name)?;

    Ok(Json(FileResponse {
        name: dir_name.clone(),
        path: state.base_dir.join(&dir_name).display().to_string(),
    }))
}

pub async fn upload_file(
    State(state): State<AppState>,
    Json(payload): Json<UploadFileRequest>,
) -> Result<Json<FileResponse>, ApiError> {
    let saved = fs_ops::save_without_replacing_files(&state.base_dir, &[(payload.name.as_str(), payload.content.as_bytes())])?;
    let path = saved.first().expect("should save a file");

    Ok(Json(FileResponse {
        name: payload.name,
        path: path.display().to_string(),
    }))
}

pub async fn replace_file(
    State(state): State<AppState>,
    Json(payload): Json<UploadFileRequest>,
) -> Result<Json<FileResponse>, ApiError> {
    let path = fs_ops::save_and_replace_file(&state.base_dir, &payload.name, payload.content.as_bytes())?;

    Ok(Json(FileResponse {
        name: payload.name,
        path: path.display().to_string(),
    }))
}

pub async fn list_files(
    State(state): State<AppState>,
    Query(query): Query<ListFilesQuery>,
) -> Result<Json<Vec<FileResponse>>, ApiError> {
    let target_dir = match query.dir {
        Some(dir) => state.base_dir.join(dir),
        None => state.base_dir.clone(),
    };

    let files = fs_ops::list_files(&target_dir)?;
    let response = files
        .into_iter()
        .map(|path| FileResponse {
            name: path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
            path: path.display().to_string(),
        })
        .collect();

    Ok(Json(response))
}

pub async fn delete_file(
    State(state): State<AppState>,
    Path(file_name): Path<String>,
) -> Result<StatusCode, ApiError> {
    fs_ops::delete_file(&state.base_dir, &file_name)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_directory(
    State(state): State<AppState>,
    Path(dir_name): Path<String>,
) -> Result<StatusCode, ApiError> {
    fs_ops::delete_dir(&state.base_dir, &dir_name)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn rename_directory(
    State(state): State<AppState>,
    Path(old_name): Path<String>,
    Json(payload): Json<RenameDirectoryRequest>,
) -> Result<Json<DirectoryResponse>, ApiError> {
    fs_ops::change_dir_name(&state.base_dir, &old_name, &payload.new_name)?;

    Ok(Json(DirectoryResponse {
        name: payload.new_name.clone(),
        path: state.base_dir.join(&payload.new_name).display().to_string(),
    }))
}

pub async fn move_directory(
    State(state): State<AppState>,
    Path(dir_name): Path<String>,
    Json(payload): Json<MoveDirectoryRequest>,
) -> Result<Json<DirectoryResponse>, ApiError> {
    let destination_base = state.base_dir.join(&payload.destination);
    fs_ops::move_dir(&state.base_dir, &dir_name, &destination_base)?;

    Ok(Json(DirectoryResponse {
        name: dir_name.clone(),
        path: destination_base.join(&dir_name).display().to_string(),
    }))
}

pub async fn save_single_file(
    State(state): State<AppState>,
    Json(payload): Json<UploadFileRequest>,
) -> Result<Json<FileResponse>, ApiError> {
    let path = fs_ops::save_file(&state.base_dir, &payload.name, payload.content.as_bytes())?;
    Ok(Json(FileResponse {
        name: payload.name,
        path: path.display().to_string(),
    }))
}

pub async fn upload_files_batch(
    State(state): State<AppState>,
    Json(payload): Json<BatchUploadFileRequest>,
) -> Result<Json<Vec<FileResponse>>, ApiError> {
    let files: Vec<(&str, &[u8])> = payload
        .files
        .iter()
        .map(|file| (file.name.as_str(), file.content.as_bytes()))
        .collect();

    let saved = fs_ops::save_files(&state.base_dir, &files)?;
    let response = saved
        .into_iter()
        .map(|path| FileResponse {
            name: path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
            path: path.display().to_string(),
        })
        .collect();

    Ok(Json(response))
}

pub async fn upload_files_batch_no_replace(
    State(state): State<AppState>,
    Json(payload): Json<BatchUploadFileRequest>,
) -> Result<Json<Vec<FileResponse>>, ApiError> {
    let files: Vec<(&str, &[u8])> = payload
        .files
        .iter()
        .map(|file| (file.name.as_str(), file.content.as_bytes()))
        .collect();

    let saved = fs_ops::save_without_replacing_files(&state.base_dir, &files)?;
    let response = saved
        .into_iter()
        .map(|path| FileResponse {
            name: path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
            path: path.display().to_string(),
        })
        .collect();

    Ok(Json(response))
}

#[derive(Debug)]
pub struct ApiError(AppError);

impl From<AppError> for ApiError {
    fn from(value: AppError) -> Self {
        Self(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self.0 {
            AppError::AlreadyExists(_) => StatusCode::CONFLICT,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.0.to_string()).into_response()
    }
}
