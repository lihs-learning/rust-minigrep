use std::fs;
use std::error::Error;

// 配置解析
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("缺少参数")
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{ query, filename })
    }
}

// 运行逻辑
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("文件内容为：\n{}", contents);

    Ok(())
}
