/*
Libreria que permite realizar todas las acciones que debe hacer la aplicacion. 
(Crear, eliminar, mover, renombrar, etc. archivos o carpetas.)
*/

use std::path::PathBuf;
use crate::errors::{AppError, Result};
use fs_more::directory::{self, DirectoryMoveOptions};

/* Funciones para operaciones con directorios */
pub fn create_new_dir(base_path: &PathBuf, dir_name: &str) -> Result<()> {
    let path = base_path.join(dir_name);

    if path.is_dir() {
        println!("El directorio ya existe: {:?}", path);
        return Err(AppError::AlreadyExists(format!("{:?}", path)));
    }

    println!("Creando el directorio: {:?}", path);
    std::fs::create_dir(&path)?;
    Ok(())
}

pub fn delete_dir(base_path: &PathBuf, dir_name: &str) -> Result<()> {
    let full_path = base_path.join(dir_name);
    if full_path.is_dir() {
        std::fs::remove_dir(&full_path)?;
    } else {
        println!("El directorio no existe: {:?}", full_path);
    }
    Ok(())
}

pub fn change_dir_name(base_path: &PathBuf, old_name: &str, new_name: &str) -> Result<()> {
    let old_path = base_path.join(old_name);
    let new_path = base_path.join(new_name);

    if old_path.is_dir() {
        std::fs::rename(&old_path, &new_path)?;
        println!("Directorio renombrado de {:?} a {:?}", old_path, new_path);
    } else {
        println!("El directorio no existe: {:?}", old_path);
        return Err(AppError::NotFound(format!("{:?}", old_path)));
    }
    Ok(())
}

pub fn move_dir(old_path: &PathBuf, dir_name: &str, new_path: &PathBuf) -> Result<()> {
    let path = old_path.join(dir_name);
    println!("=============================Direccion: {:?}==================================", old_path);

    if !new_path.join(dir_name).is_dir() {
        create_new_dir(&new_path, dir_name)?;
    }
    println!("=============================Direccion: {:?}==================================", new_path);
    directory::move_directory(&path, &new_path.join(dir_name), DirectoryMoveOptions::default())
        .map(|_finished| ())
        .map_err(|e| AppError::Other(format!("Error moviendo directorio: {e}")))?;
    Ok(())
}

pub fn list_dirs(base_path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();

    if base_path.is_dir() {
        for entry in std::fs::read_dir(base_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            }
        }
    } else {
        println!("El directorio no existe: {:?}", base_path);
        return Err(AppError::NotFound(format!("{:?}", base_path)));
    }

    Ok(dirs)
}

pub fn get_dir_metadata(base_path: &PathBuf, dir_name: &str) -> Result<std::fs::Metadata> {
    let path = base_path.join(dir_name);

    if path.exists() {
        let metadata = std::fs::metadata(&path)?;
        Ok(metadata)
    } else {
        println!("El directorio no existe: {:?}", path);
        Err(AppError::NotFound(format!("{:?}", path)))
    }
}

pub fn get_dir_size(base_path: &PathBuf, dir_name: &str) -> Result<u64> {
    let metadata = get_dir_metadata(base_path, dir_name)?;
    Ok(metadata.len())
}



/* Funciones para operaciones con archivos */
pub fn save_file(base_path: &PathBuf, file_name: &str, file: &[u8]) -> Result<PathBuf> {
    let path = base_path.join(file_name);

    if path.is_file() {
        println!("El archivo ya existe: {:?}", path);
        return Err(AppError::AlreadyExists(format!("{:?}", path)));
    }

    if !base_path.is_dir() {
        println!("Creando la carpeta de destino: {:?}", base_path);
        std::fs::create_dir_all(base_path)?;
    }

    println!("Guardando archivo: {:?}", path);
    std::fs::write(&path, file)?;
    Ok(path)
}

pub fn save_files(base_path: &PathBuf, files: &[(&str, &[u8])]) -> Result<Vec<PathBuf>> {
    if !base_path.is_dir() {
        println!("Creando la carpeta de destino: {:?}", base_path);
        std::fs::create_dir_all(base_path)?;
    }

    let mut saved_paths = Vec::new();

    for (file_name, bytes_file) in files {
        let path = base_path.join(file_name);

        if path.is_file() {
            println!("El archivo ya existe: {:?}", path);
            return Err(AppError::AlreadyExists(format!("{:?}", path)));
        }

        println!("Guardando archivo: {:?}", path);
        std::fs::write(&path, bytes_file)?;
        saved_paths.push(path);
    }

    Ok(saved_paths)
}

pub fn delete_file(base_path: &PathBuf, file_name: &str) -> Result<()> {
    let full_path = base_path.join(file_name);
    if full_path.is_file() { 
        std::fs::remove_file(&full_path)?; 
    } else {
        println!("El archivo no existe: {:?}", full_path);
    }
    Ok(())
}

pub fn list_files(base_path: &PathBuf) -> Result<Vec<PathBuf>> {
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
        return Err(AppError::NotFound(format!("{:?}", base_path)));
    }

    Ok(files)
}

pub fn save_and_replace_file(base_path: &PathBuf, file_name: &str, file: &[u8]) -> Result<PathBuf> {
    let path = base_path.join(file_name);

    if !base_path.is_dir() {
        println!("Creando la carpeta de destino: {:?}", base_path);
        std::fs::create_dir_all(base_path)?;
    }

    println!("Guardando archivo (reemplazando si existe): {:?}", path);
    std::fs::write(&path, file)?;
    Ok(path)
}

pub fn save_without_replacing_files(base_path: &PathBuf, files: &[(&str, &[u8])]) -> Result<Vec<PathBuf>> {
    if !base_path.is_dir() {
        println!("Creando la carpeta de destino: {:?}", base_path);
        std::fs::create_dir_all(base_path)?;
    }

    let mut saved_paths = Vec::new();

    for (file_name, bytes_file) in files {
        let path = base_path.join(file_name);

        if path.is_file() {
            println!("El archivo ya existe: {:?}", path);
            return Err(AppError::AlreadyExists(format!("{:?}", path)));
        }

        println!("Guardando archivo: {:?}", path);
        std::fs::write(&path, bytes_file)?;
        saved_paths.push(path);
    }

    Ok(saved_paths)
}

pub fn move_file(old_path: &PathBuf, file_name: &str, new_path: &PathBuf) -> Result<()> {
    let path = old_path.join(file_name);
    println!("=============================Direccion: {:?}==================================", old_path);

    if !new_path.is_dir() {
        println!("Creando la carpeta de destino: {:?}", new_path);
        std::fs::create_dir_all(new_path)?;
    }

    println!("=============================Direccion: {:?}==================================", new_path);
    std::fs::rename(&path, &new_path.join(file_name))?;
    Ok(())
}

pub fn get_metadata(base_path: &PathBuf, file_name: &str) -> Result<std::fs::Metadata> {
    let path = base_path.join(file_name);

    if path.exists() {
        let metadata = std::fs::metadata(&path)?;
        Ok(metadata)
    } else {
        println!("El archivo no existe: {:?}", path);
        Err(AppError::NotFound(format!("{:?}", path)))
    }
}

pub fn get_file_size(base_path: &PathBuf, file_name: &str) -> Result<u64> {
    let metadata = get_metadata(base_path, file_name)?;
    Ok(metadata.len())
}

pub fn rename_file(base_path: &PathBuf, old_name: &str, new_name: &str) -> Result<()> {
    let old_path = base_path.join(old_name);
    let new_path = base_path.join(new_name);

    if old_path.is_file() {
        std::fs::rename(&old_path, &new_path)?;
        println!("Archivo renombrado de {:?} a {:?}", old_path, new_path);
    } else {
        println!("El archivo no existe: {:?}", old_path);
        return Err(AppError::NotFound(format!("{:?}", old_path)));
    }
    Ok(())
}


/* Filtros y busquedas */
pub fn filter_files_by_extension(base_path: &PathBuf, extension: &str) -> Result<Vec<PathBuf>> {
    let mut filtered_files = Vec::new();

    if base_path.is_dir() {
        for entry in std::fs::read_dir(base_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == extension {
                        filtered_files.push(path);
                    }
                }
            }
        }
    } else {
        println!("El directorio no existe: {:?}", base_path);
        return Err(AppError::NotFound(format!("{:?}", base_path)));
    }

    Ok(filtered_files)
}

pub fn search_files_by_name(base_path: &PathBuf, name: &str) -> Result<Vec<PathBuf>> {
    let mut found_files = Vec::new();

    if base_path.is_dir() {
        for entry in std::fs::read_dir(base_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if file_name.to_string_lossy().contains(name) {
                        found_files.push(path);
                    }
                }
            }
        }
    } else {
        println!("El directorio no existe: {:?}", base_path);
        return Err(AppError::NotFound(format!("{:?}", base_path)));
    }

    Ok(found_files)
}

pub fn search_dirs_by_name(base_path: &PathBuf, name: &str) -> Result<Vec<PathBuf>> {
    let mut found_dirs = Vec::new();

    if base_path.is_dir() {
        for entry in std::fs::read_dir(base_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name() {
                    if dir_name.to_string_lossy().contains(name) {
                        found_dirs.push(path);
                    }
                }
            }
        }
    } else {
        println!("El directorio no existe: {:?}", base_path);
        return Err(AppError::NotFound(format!("{:?}", base_path)));
    }

    Ok(found_dirs)
}

pub fn order_files_by_size(base_path: &PathBuf, ascending: bool) -> Result<Vec<PathBuf>> {
    let mut files = list_files(base_path)?;

    files.sort_by(|a, b| {
        let size_a = get_file_size(base_path, a.file_name().unwrap().to_str().unwrap()).unwrap_or(0);
        let size_b = get_file_size(base_path, b.file_name().unwrap().to_str().unwrap()).unwrap_or(0);
        if ascending {
            size_a.cmp(&size_b)
        } else {
            size_b.cmp(&size_a)
        }
    });

    Ok(files)
}

pub fn order_dirs_by_size(base_path: &PathBuf, ascending: bool) -> Result<Vec<PathBuf>> {
    let mut dirs = list_dirs(base_path)?;

    dirs.sort_by(|a, b| {
        let size_a = get_dir_size(base_path, a.file_name().unwrap().to_str().unwrap()).unwrap_or(0);
        let size_b = get_dir_size(base_path, b.file_name().unwrap().to_str().unwrap()).unwrap_or(0);
        if ascending {
            size_a.cmp(&size_b)
        } else {
            size_b.cmp(&size_a)
        }
    });

    Ok(dirs)
}

pub fn order_files_by_name(base_path: &PathBuf, ascending: bool) -> Result<Vec<PathBuf>> {
    let mut files = list_files(base_path)?;

    files.sort_by(|a, b| {
        let name_a = a.file_name().unwrap().to_string_lossy();
        let name_b = b.file_name().unwrap().to_string_lossy();
        if ascending {
            name_a.cmp(&name_b)
        } else {
            name_b.cmp(&name_a)
        }
    });

    Ok(files)
}

pub fn order_dirs_by_name(base_path: &PathBuf, ascending: bool) -> Result<Vec<PathBuf>> {
    let mut dirs = list_dirs(base_path)?;

    dirs.sort_by(|a, b| {
        let name_a = a.file_name().unwrap().to_string_lossy();
        let name_b = b.file_name().unwrap().to_string_lossy();
        if ascending {
            name_a.cmp(&name_b)
        } else {
            name_b.cmp(&name_a)
        }
    });

    Ok(dirs)
}

pub fn order_by_modification_time(base_path: &PathBuf, ascending: bool) -> Result<Vec<PathBuf>> {
    let mut entries = Vec::new();

    if base_path.is_dir() {
        for entry in std::fs::read_dir(base_path)? {
            let entry = entry?;
            let path = entry.path();
            entries.push(path);
        }
    } else {
        println!("El directorio no existe: {:?}", base_path);
        return Err(AppError::NotFound(format!("{:?}", base_path)));
    }

    entries.sort_by(|a, b| {
        let meta_a = std::fs::metadata(a).ok();
        let meta_b = std::fs::metadata(b).ok();

        let time_a = meta_a.and_then(|m| m.modified().ok());
        let time_b = meta_b.and_then(|m| m.modified().ok());

        match (time_a, time_b) {
            (Some(t_a), Some(t_b)) => {
                if ascending {
                    t_a.cmp(&t_b)
                } else {
                    t_b.cmp(&t_a)
                }
            }
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });

    Ok(entries)
}

pub fn search_by_name(base_path: &PathBuf, name: &str) -> Result<Vec<PathBuf>> {
    let mut found_entries = Vec::new();

    if base_path.is_dir() {
        for entry in std::fs::read_dir(base_path)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(entry_name) = path.file_name() {
                if entry_name.to_string_lossy().contains(name) {
                    found_entries.push(path);
                }
            }
        }
    } else {
        println!("El directorio no existe: {:?}", base_path);
        return Err(AppError::NotFound(format!("{:?}", base_path)));
    }

    Ok(found_entries)
}

