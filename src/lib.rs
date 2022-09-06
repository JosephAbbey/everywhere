macro_rules! everywhere {
    (
        $(pub fn $fname:ident($($arg:ident: $arg_t:ty),*) -> $t:ty $body:block)*
    ) => {
        $(pub fn $fname($($arg: $arg_t),*) -> $t $body)*
    };
}

include!("./index.rs");
