/*
Libreria que permite realizar todas las acciones que debe hacer la aplicacion. 
(Crear, eliminar, mover, renombrar, etc. archivos o carpetas.)
*/

use std::path::PathBuf;
use fs_more::directory::{self, DirectoryMoveOptions};

/* Funciones para operaciones con directorios */
pub fn create_new_dir(base_path: &PathBuf, dir_name: &str) -> Result<(), std::io::Error> {
    let path = base_path.join(dir_name);

    if path.is_dir() {
        println!("El directorio ya existe: {:?}", path);
        return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "El directorio ya existe"));
    } else {
        println!("Creando el directorio: {:?}", path);
        std::fs::create_dir(&path)?;
        Ok(())
    }
    
}

pub fn delete_dir(base_path: &PathBuf, dir_name: &str) -> Result<(), std::io::Error> {
    let full_path = base_path.join(dir_name);
    if full_path.is_dir() { 
    std::fs::remove_dir(&full_path)?;} 
    else{
        println!("El directorio no existe: {:?}", full_path);
    }
    Ok(())
}

pub fn change_dir_name(base_path: &PathBuf, old_name: &str, new_name: &str) -> Result<(), std::io::Error> {
    let old_path = base_path.join(old_name);
    let new_path = base_path.join(new_name);

    if old_path.is_dir() {
        std::fs::rename(&old_path, &new_path)?;
        println!("Directorio renombrado de {:?} a {:?}", old_path, new_path);
    } else {
        println!("El directorio no existe: {:?}", old_path);
    }
    Ok(())
}

pub fn move_dir(old_path: &PathBuf, dir_name: &str, new_path: &PathBuf) -> Result<(), std::io::Error>{
    let path = old_path.join(dir_name);
    println!("=============================Direccion: {:?}==================================", old_path);
    
        if !new_path.join(dir_name).is_dir(){
            create_new_dir(&new_path, dir_name).unwrap();
        }
        println!("=============================Direccion: {:?}==================================", new_path);
        directory::move_directory(&path, &new_path.join(dir_name), DirectoryMoveOptions::default())
        .map(|_finished| ())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(())
    }

/* Funciones para operaciones con archivos */
pub fn save_file(base_path: &PathBuf, file_name: &str, file: &[u8]) -> Result<PathBuf, std::io::Error> {
    let path = base_path.join(file_name);

    if path.is_file() {
        println!("El archivo ya existe: {:?}", path);
        return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "El archivo ya existe"));
    } else {
        if !base_path.is_dir() {
            println!("Creando la carpeta de destino: {:?}", base_path);
            std::fs::create_dir_all(base_path)?;
        }

        println!("Guardando archivo: {:?}", path);
        std::fs::write(&path, file)?;
        Ok(path)
    }
}

pub fn save_files(base_path: &PathBuf, files: &[(&str, &[u8])]) -> Result<Vec<PathBuf>, std::io::Error> {
    if !base_path.is_dir() {
        println!("Creando la carpeta de destino: {:?}", base_path);
        std::fs::create_dir_all(base_path)?;
    }

    let mut saved_paths = Vec::new();

    for (file_name, bytes_file) in files {
        let path = base_path.join(file_name);

        if path.is_file() {
            println!("El archivo ya existe: {:?}", path);
            return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "El archivo ya existe"));
        }

        println!("Guardando archivo: {:?}", path);
        std::fs::write(&path, bytes_file)?;
        saved_paths.push(path);
    }

    Ok(saved_paths)
}

pub fn delete_file(base_path: &PathBuf, file_name: &str) -> Result<(), std::io::Error> {
    let full_path = base_path.join(file_name);
    if full_path.is_file() { 
        std::fs::remove_file(&full_path)?; 
    } else {
        println!("El archivo no existe: {:?}", full_path);
    }
    Ok(())
}

pub fn list_files(base_path: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();

    if base_path.is_dir() {
        for entry in std::fs::read_dir(base_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
    } else {
        println!("El directorio no existe: {:?}", base_path);
    }

    Ok(files)
}