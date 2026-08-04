use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateDirRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadFileRequest {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct BatchUploadFileRequest {
    pub files: Vec<UploadFileRequest>,
}

#[derive(Debug, Deserialize)]
pub struct RenameDirectoryRequest {
    pub new_name: String,
}

#[derive(Debug, Deserialize)]
pub struct MoveDirectoryRequest {
    pub destination: String,
}

#[derive(Debug, Deserialize)]
pub struct ListFilesQuery {
    pub dir: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FileResponse {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct DirectoryResponse {
    pub name: String,
    pub path: String,
}
