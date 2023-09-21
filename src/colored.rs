use once_cell::sync::OnceCell;

pub static USE_VERBOSE: OnceCell<bool> = OnceCell::new();

macro_rules! styled {
    ($stream:expr, $style:expr, $($arg:tt)*) => {{
        use std::io::Write;
        let mut stream = $stream.lock();
        let _ = ::std::write!(&mut stream, "{}", $style.render());
        let _ = ::std::write!(&mut stream, $($arg)*);
        let _ = ::std::writeln!(&mut stream, "{}", $style.render_reset());
    }};
}

macro_rules! verbose {
    ($($arg:tt)*) => {{
        if let Some(true) = crate::colored::USE_VERBOSE.get() {
            styled!(
                ::anstream::stdout(),
                ::anstyle::Style::new().dimmed(),
                $($arg)*
            )
        }
    }};
}

macro_rules! info {
    ($($arg:tt)*) => {{
        styled!(
            ::anstream::stdout(),
            ::anstyle::Style::new().bold(),
            $($arg)*
        )
    }};
}

macro_rules! important {
    ($($arg:tt)*) => {{
        styled!(
            ::anstream::stdout(),
            ::anstyle::AnsiColor::Cyan.on_default(),
            $($arg)*
        )
    }};
}

macro_rules! warning {
    ($($arg:tt)*) => {{
        styled!(
            ::anstream::stderr(),
            ::anstyle::AnsiColor::Yellow.on_default(),
            $($arg)*
        )
    }};
}

macro_rules! error {
    ($($arg:tt)*) => {{
        styled!(
            ::anstream::stderr(),
            ::anstyle::AnsiColor::Red.on_default(),
            $($arg)*
        )
    }};
}
