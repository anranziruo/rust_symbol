mod read_dmp;

#[tokio::main]
async fn main() -> Result<(),()>{
   let mut dmp_item = read_dmp::ReadDmp{
        dmp_path:"test_data/test.dmp".to_string(),
   };
   let dmp_result =dmp_item.read_mini_dmp().await;
   match dmp_result {
      Ok(v) => println!("working with version: {v:?}"),
      Err(e) => println!("error parsing header: {e:?}"),
   }
   Ok(())
}
