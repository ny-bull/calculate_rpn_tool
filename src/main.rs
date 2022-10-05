//use std::env;
mod derive;
fn main() {
    //自前で引数を受け取るプログラムを記述する場合
    //let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    derive::run();
}
