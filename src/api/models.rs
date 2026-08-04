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
