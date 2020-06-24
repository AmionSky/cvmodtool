use termcolored::*;

pub static mut USE_VERBOSE: bool = true;

pub fn verbose(text: &str) {
    if unsafe { USE_VERBOSE } {
        println!("{}", text.color(Color::BrightBlack));
    }
}

pub fn info(text: &str) {
    println!("{}", text.white());
}

pub fn warning(text: &str) {
    println!("{}", text.yellow());
}

pub fn error(text: &str) {
    eprintln!("{}", text.red());
}
