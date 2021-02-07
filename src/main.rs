use leptess::*;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    if (args.len() < 2){
        println!("Usage: rust-ocr-test [path]");
        return;
    }

    let mut lt = leptess::LepTess::new(Some("./data"), "eng").unwrap();
    lt.set_image(&args[1]);
    println!("{}", lt.get_utf8_text().unwrap());
}
