use std::path::PathBuf;
use cfg::prelude::*;
use luban_lib::ByteBuf;

/// 加载配置表的公共函数
/// 
/// # 参数
/// * `base_path` - 配置文件的基础路径
/// ```
pub fn load_tables(base_path: &str) -> Tables {
    Tables::new(|name| {
        let path = PathBuf::from(format!("{}/{}.bytes", base_path, name));
        Ok(ByteBuf::new(std::fs::read(path).unwrap()))
    }).expect("Failed to load tables")
}

/// 使用默认路径加载配置表的函数
pub fn load_tables_default() -> Tables {
    load_tables("../GenerateDatas/bytes")
}