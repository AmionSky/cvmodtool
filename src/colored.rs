use color_print::{cprintln, cformat};

pub static mut USE_VERBOSE: bool = true;

pub fn verbose(text: &str) {
    if unsafe { USE_VERBOSE } {
        cprintln!("<dim>{}</>", text);
    }
}

pub fn important(text: &str) {
    cprintln!("<cyan>{}</>", text);
}

pub fn info(text: &str) {
    cprintln!("<white>{}</>", text);
}

pub fn warning(text: &str) {
    cprintln!("<yellow>{}</>", text);
}

pub fn error(text: &str) {
    eprintln!("{}", cformat!("<red>{}</>", text));
}
