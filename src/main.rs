mod read_dmp;




fn main() {
   let mut dmp_item = read_dmp::ReadDmp{
        dmp_path:"234".to_string(),
   };
   let dmp_result = dmp_item.get_dmp_module();
   match dmp_result {
    Ok(v) => println!("working with version: {v:?}"),
    Err(e) => println!("error parsing header: {e:?}"),
    }
}
