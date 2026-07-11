/*
Libreria que permite realizar todas las acciones que debe hacer la aplicacion. 
(Crear, eliminar, mover, renombrar, etc. archivos o carpetas.)
*/

use std::path::PathBuf;

pub fn create_new_dir(path: &PathBuf) -> Result<(), std::io::Error> {

    println!("La direccion es: {:?}<-------------------", path);
    std::fs::create_dir_all(&path)?;
    Ok(())
}
