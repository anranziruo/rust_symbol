#[test]
fn test_debug_id(){
    let debug_none:Option<String> = None;
    println!("{:?}",debug_none.unwrap_or_default())
}