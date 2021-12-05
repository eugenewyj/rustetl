use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    // let parttern = std::env::args().nth(1).expect("未指定要搜索的匹配模式");
    // let path = std::env::args().nth(2).expect("未指定要搜到的文件路径");

    // let cli = Cli {
    //         pattern: parttern,
    //         path: std::path::PathBuf::from(path),
    //     };
    // println!("要搜索的匹配模式是:{:?}", parttern);
    // println!("要搜索的文件路径是:{:?}", path);

    let cli = Cli::from_args();
    println!("参数是：{:?}", cli);

    let content = std::fs::read_to_string(&cli.path).expect("不能读取文件");
    for line in content.lines() {
        if line.contains(&cli.pattern) {
            println!("{}", line);
        }
    }
}
