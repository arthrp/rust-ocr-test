use leptess::*;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    if (args.len() < 2){
        println!("Usage: rust-ocr-test [path to file] -f [path to models folder]");
        return;
    }

    let lang_folder = if args.len() > 2 && args[2] == "-f" { &args[3] } else { "./data" };
    let mut lt = leptess::LepTess::new(Some(lang_folder), "eng").unwrap();
    lt.set_image(&args[1]);
    println!("{}", lt.get_utf8_text().unwrap());
}
