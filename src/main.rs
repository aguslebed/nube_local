use std::path::PathBuf;
use nube_local::fs_ops;
use std::path::Path;

const BASE_DIR_NAME: &str = "nube-data";

fn get_root_path() -> PathBuf {
    dirs::home_dir()
        .expect("no se pudo encontrar el home")
}

fn create_base_dir() -> Result<(), std::io::Error> {
    let root_path = get_root_path();
    if root_path.is_dir() {
        println!("El directorio base ya existe: {:?}", root_path);
    } else {
        println!("Creando el directorio base: {:?}", root_path);
        fs_ops::create_new_dir(&root_path, BASE_DIR_NAME).expect("Error al crear el directorio base");
    }
     Ok(())
}

fn main() {
    create_base_dir().expect("Error al crear el directorio base");
    let root_path = get_root_path().join(BASE_DIR_NAME);
  
    //fs_ops::create_new_dir(&root_path, "test_dir").unwrap();
    //fs_ops::delete_dir(&root_path, "test_dir").unwrap();
    fs_ops::change_dir_name(&root_path, "test_dir", "Carpeta de prueba").expect("Error al cambiar el nombre de la carpeta");
    println!("Root path: {:?}", root_path.join("Carpeta de prueba"));
}