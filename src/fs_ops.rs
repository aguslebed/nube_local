/*
Libreria que permite realizar todas las acciones que debe hacer la aplicacion. 
(Crear, eliminar, mover, renombrar, etc. archivos o carpetas.)
*/

use std::path::PathBuf;
use fs_more::directory::{self, DirectoryMoveOptions};

pub fn create_new_dir(base_path: &PathBuf, dir_name: &str) -> Result<(), std::io::Error> {
    let path = base_path.join(dir_name);

    if path.is_dir() {
        println!("El directorio ya existe: {:?}", path);
    } else {
        println!("Creando el directorio: {:?}", path);
    }
    std::fs::create_dir(&path)?;
    Ok(())
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

pub fn move_dir(base_path: &PathBuf, dir_name: &str, new_path: &PathBuf) -> Result<(), std::io::Error>{
    let old_path = base_path.join(dir_name);
    let new_full_path = new_path.join(dir_name);
    directory::move_directory(&old_path, &new_full_path, DirectoryMoveOptions::default())
        .map(|_finished| ())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}