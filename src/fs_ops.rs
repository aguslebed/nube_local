/*
Libreria que permite realizar todas las acciones que debe hacer la aplicacion. 
(Crear, eliminar, mover, renombrar, etc. archivos o carpetas.)
*/

use std::path::PathBuf;

pub fn create_new_dir(path: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let full_path = path.join(name);
    std::fs::create_dir(&full_path)?;
    Ok(())
}
