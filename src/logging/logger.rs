#[macro_export]
macro_rules! log {
    //case 1: empty
    () => {
        $crate::interop::imports::log_str(&"".to_string());
    };

    //case 2: single argument
    ($msg:expr) => {
        let string = std::format!("{}", $msg).to_string();
        $crate::interop::imports::log_str(&string);
    };

    //case 3: multiple arguments
    ($($arg:tt)*) => {{
        let string = &format_args!($($arg)*).to_string();
        $crate::interop::imports::log_str(&string);
    }};
}