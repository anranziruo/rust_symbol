
use std::path::PathBuf;

use rust_symbol::download_binary_file;

#[test]
fn test_get_file_name(){
    let path = PathBuf::from("c:\\test_app.pdb".replace("\\", "/"));
    println!("{:?}",path.file_name())
}

#[tokio::test]
async fn test_download_binary_file(){
    let data = download_binary_file(&"https://github.com/rust-minidump/rust-minidump/raw/main/testdata/test.dmp").await;
    match data {
           Ok(v) => {
                
           },
           Err(e) => println!("occur err: {e:?}"),
    }
}