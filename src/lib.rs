macro_rules! everywhere {
    (
        $(pub fn $fname:ident($($arg:ident: $arg_t:ty),*) -> $t:ty $body:block)*
    ) => {
        $(pub fn $fname($($arg: $arg_t),*) -> $t $body)*
    };
}

include!("./index.rs");

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn hi_works() {
        assert_eq!(hi("Joseph".to_string()), "Hi Joseph!".to_string());
    }

    #[test]
    fn hello_works() {
        assert_eq!(hello("Joseph".to_string(), "Abbey".to_string()), "Hello Joseph Abbey.".to_string());
    }

    #[test]
    fn add_works() {
        assert_eq!(add(1f64, 4f64), 5f64);
    }
}
