use std::env;

pub fn debug(msg: String) {
    if env::var("DEBUG").is_ok() {
        println!("{}", msg);
    }
}
