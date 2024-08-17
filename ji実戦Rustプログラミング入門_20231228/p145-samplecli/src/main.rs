use std::env;

fn main() {
    // コマンドライン引数をベクターに収集
    let args: Vec<String> = env::args().collect();

    // "HelloWorld"と出力
    println!("HelloWorld");

    // 収集したコマンドライン引数をデバッグ用に表示
    println!("{:?}", args);
}