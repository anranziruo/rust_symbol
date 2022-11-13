mod read_dmp;
use tokio::task;
fn main() {
   let mut dmp_item = read_dmp::ReadDmp{
        dmp_path:"test_data/test.dmp".to_string(),
   };

   let dmp_result =dmp_item.read_mini_dmp();
   println!("{:?}",dmp_result);
}
