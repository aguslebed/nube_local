/*
Libreria que permite realizar todas las acciones que debe hacer la aplicacion. 
(Crear, eliminar, mover, renombrar, etc. archivos o carpetas.)
*/

use std::path::PathBuf;
use axum::extract::path;
use fs_more::{directory::{self, DirectoryMoveOptions}, error};

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