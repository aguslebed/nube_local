use std::path::PathBuf;
use nube_local::fs_ops;
use std::path::Path;

const BASE_DIR_NAME: &str = "nube-data";

fn get_root_path() -> PathBuf {
    dirs::home_dir()
        .expect("no se pudo encontrar el home")
        .join(BASE_DIR_NAME)
}

fn create_base_dir() -> Result<(), std::io::Error> {
    let root_path = get_root_path();
    std::path::Path::is_dir(&root_path).then(|| {
        println!("El directorio base ya existe: {:?}", root_path);
    }).unwrap_or_else(|| {
        println!("Creando el directorio base: {:?}", root_path);
        fs_ops::create_new_dir(&root_path.join(BASE_DIR_NAME)).expect("sadasd");  
    });
     Ok(())
}

fn main() {
    create_base_dir().expect("Error al crear el directorio base");
    let root_path = get_root_path();
    let full_path = root_path.join("test_dir");
    fs_ops::create_new_dir(&full_path).unwrap();
    println!("Root path: {:?}", root_path);
}