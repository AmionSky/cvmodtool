pub static mut USE_VERBOSE: bool = true;

pub fn verbose(text: &str) {
    if unsafe { USE_VERBOSE } {
        colour::grey_ln!("{}", text);
    }
}

pub fn important(text: &str) {
    colour::cyan_ln!("{}", text);
}

pub fn info(text: &str) {
    colour::white_ln!("{}", text);
}

pub fn warning(text: &str) {
    colour::yellow_ln!("{}", text);
}

pub fn error(text: &str) {
    colour::e_red_ln!("{}", text);
}
