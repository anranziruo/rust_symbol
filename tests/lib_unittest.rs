use std::path::PathBuf;

#[test]
fn test_get_file_name(){
    let path = PathBuf::from("c:\\test_app.pdb".replace("\\", "/"));
    println!("{:?}",path.file_name())
}