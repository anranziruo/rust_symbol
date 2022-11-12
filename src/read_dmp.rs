use minidump::Minidump;
use minidump_processor::{ProcessorOptions,Symbolizer};

#[derive(Debug, Clone)]
pub struct DmpModule {
//pdb信息
    pub debug_id: String,
    pub appendix :u32,
    pub debug_identifier:String,
//code信息
    pub code_identifier:String,
}

pub struct ReadDmp{
    
}


impl ReadDmp {
    pub fn get_dmp_module(){
        
    }
}
