use minidump::*;
use minidump_processor::{ProcessorOptions,Symbolizer,symbols};
use anyhow::{self};
use rust_symbol::get_file_name;
use serde::{Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone,Serialize)]
pub struct DmpModule {
//pdb信息
    pub debug_id: String,
    pub appendix :u32,
    pub debug_file:String,
//code信息
    pub code_file:String,
    pub code_version:String,
//地址信息
    pub start_address:String,
    pub end_address:String
}

#[derive(Debug, Clone,Serialize)]
pub struct DmpInfos {
    pub content:String
}


pub struct ReadDmp{
    pub dmp_path:String
}


impl ReadDmp {

    //读取dmp的符号信息
    pub fn get_dmp_module(self)-> Result<Vec<DmpModule>,anyhow::Error> {
        let mut dump = minidump::Minidump::read_path(self.dmp_path)?;
        let mut module_lists = Vec::new();
        if let Ok(modules) = dump.get_stream::<MinidumpModuleList>() {
            for module_item in modules.iter(){
                module_lists.push(DmpModule {
                    debug_id:module_item.debug_identifier().unwrap_or_default().uuid().to_string(),
                    appendix:module_item.debug_identifier().unwrap_or_default().appendix(),
                    debug_file:get_file_name(module_item.debug_file().unwrap_or_default().to_string()),
                    code_file:get_file_name(module_item.code_file().to_string()),
                    start_address:format!("{:#018X}", module_item.base_address()),
                    end_address:format!("{:#018X}",module_item.base_address()+module_item.size()),
                    code_version:module_item.version().unwrap_or_default().to_string()
                });
            }
        }
        Ok(module_lists)
    }

    //读取dmp文件信息
    pub async fn read_mini_dmp(self)-> Result<DmpInfos, anyhow::Error>{

        let dmp_detail = DmpInfos{
            content:String::from("")
        };

        let dump = minidump::Minidump::read_path(self.dmp_path)?;

        //设置空路径
        let mut path = PathBuf::new();
        let symbol_paths = vec![path];
        let mut options = ProcessorOptions::default();

        let provider = Symbolizer::new(symbols::simple_symbol_supplier(symbol_paths));

        let state = minidump_processor::process_minidump(&dump, &provider)
        .await
        .map_err(|_| ());

        Ok(dmp_detail)
    }
}
