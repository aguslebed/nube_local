use std::path::PathBuf;
use nube_local::fs_ops;
use std::path::Path;
use std::io;

const BASE_DIR_NAME: &str = "nube-data";

fn get_root_path() -> PathBuf {
    dirs::home_dir()
        .expect("no se pudo encontrar el home")
}

fn create_base_dir() -> Result<(), std::io::Error> {
    let root_path = get_root_path();
    let base_path = root_path.join(BASE_DIR_NAME);
    if base_path.is_dir() {
        println!("El directorio base ya existe: {:?}", base_path);
    } else {
        println!("Creando el directorio base: {:?}", base_path);
        if let Err(e) = fs_ops::create_new_dir(&root_path, BASE_DIR_NAME) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(e);
            } else {
                println!("Aviso: el directorio base ya existía (ignorado): {:?}", base_path);
            }
        }
    }
     Ok(())
}

fn main() {
    if let Err(e) = create_base_dir() {
        eprintln!("Error al crear el directorio base: {}", e);
    }
    let root_path = get_root_path().join(BASE_DIR_NAME);

    if let Err(e) = fs_ops::create_new_dir(&root_path, "test_dir") {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            eprintln!("Error creando test_dir: {}", e);
        } else {
            println!("El directorio ya existe: {:?}", root_path.join("test_dir"));
        }
    }

    if let Err(e) = fs_ops::create_new_dir(&root_path, "Carpeta de prueba") {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            eprintln!("Error creando Carpeta de prueba: {}", e);
        } else {
            println!("El directorio ya existe: {:?}", root_path.join("Carpeta de prueba"));
        }
    }

    if let Err(e) = fs_ops::move_dir(&root_path, "Carpeta de prueba", &root_path.join("test_dir")) {
        eprintln!("Error moviendo carpeta: {}", e);
    } else {
        println!("Movimiento completado.");
    }

    println!("Root path: {:?}", root_path.join("Carpeta de prueba"));
}