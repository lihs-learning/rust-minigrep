use std::{
    env,
    process,
};

use minigrep::Config;

fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("在读取参数时发生了错误：{}", err);
        process::exit(1);
    });

    println!("要查询的字符串为：{}", config.query);
    println!("被查询的文件为：{}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("程序错误: {}", e);

        process::exit(1);
    }
}
