use std::path::Path;

pub fn get_file_name(dir_name:String) -> String{
    let new_dir = dir_name.replace("\\", "/");
    let path = Path::new(&new_dir);
    let file = path.file_name();
    return file.unwrap_or_default().to_string_lossy().to_string();
}