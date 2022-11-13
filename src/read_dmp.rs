use minidump::*;
use minidump_processor::{ProcessorOptions,Symbolizer};
use anyhow::{self};
#[derive(Debug, Clone)]
pub struct DmpModule {
//pdb信息
    pub debug_id: String,
    pub appendix :u32,
    pub debug_file:String,
//code信息
    pub code_identifier:String,
//地址信息
    pub start_address:u64,
    pub end_address:u64
}

pub struct ReadDmp{
    pub dmp_path:String
}


impl ReadDmp {
    pub fn get_dmp_module(self)-> Result<Vec<DmpModule>,anyhow::Error> {
        let mut dump = minidump::Minidump::read_path("test_data/test.dmp")?;
        let mut module_lists = Vec::new();
        if let Ok(modules) = dump.get_stream::<MinidumpModuleList>() {
            for module_item in modules.iter(){
                module_lists.push(DmpModule {
                    debug_id:module_item.debug_identifier().unwrap_or_default().uuid().to_string(),
                    appendix:module_item.debug_identifier().unwrap_or_default().appendix(),
                    debug_file:module_item.debug_file().unwrap_or_default().to_string(),
                    code_identifier:module_item.code_identifier().unwrap_or_default().to_string(),
                    start_address:module_item.base_address(),
                    end_address:module_item.base_address()+module_item.size(),
                });
            }
        }
        Ok(module_lists)
    }
}
