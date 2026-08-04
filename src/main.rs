use std::path::PathBuf;
use nube_local::api;
use nube_local::errors::AppError;
use nube_local::fs_ops;

const BASE_DIR_NAME: &str = "nube-data";

fn get_root_path() -> PathBuf {
    PathBuf::from("/")
}

fn create_base_dir() -> Result<(), AppError> {
    let root_path = get_root_path();
    let base_path = root_path.join(BASE_DIR_NAME);
    if base_path.is_dir() {
        println!("El directorio base ya existe: {:?}", base_path);
    } else {
        println!("Creando el directorio base: {:?}", base_path);
        match fs_ops::create_new_dir(&root_path, BASE_DIR_NAME) {
            Ok(()) => {}
            Err(AppError::AlreadyExists(_)) => {
                println!("Aviso: el directorio base ya existía (ignorado): {:?}", base_path);
            }
            Err(err) => return Err(err),
        }
    }
     Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match create_base_dir() {
        Ok(()) => println!("Directorio base listo."),
        Err(e) => eprintln!("Error al preparar el directorio base: {}", e),
    }

    let base_dir = get_root_path().join(BASE_DIR_NAME);
    let app = api::create_router(base_dir);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Servidor escuchando en http://0.0.0.0:3000");
    axum::serve(listener, app).await?;

    Ok(())
}