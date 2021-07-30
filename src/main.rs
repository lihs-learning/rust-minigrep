use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("在读取参数时发生了错误：{}", err);
        process::exit(1);
    });

    println!("要查询的字符串为：{}", config.query);
    println!("被查询的文件为：{}", config.filename);

    if let Err(e) = run(config) {
        println!("程序错误: {}", e);

        process::exit(1);
    }
}

// 配置解析
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("缺少参数")
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{ query, filename })
    }
}

// 运行逻辑
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("文件内容为：\n{}", contents);

    Ok(())
}
