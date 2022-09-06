macro_rules! everywhere {
    (
        $(pub fn $fname:ident($($arg:ident: $arg_t:ty),*) -> $t:ty $body:block)*
    ) => {
        pub trait Js { const JS: &'static str; }
        impl Js for String { const JS: &'static str = "string"; }
        impl Js for bool { const JS: &'static str = "boolean"; }
        impl Js for f64 { const JS: &'static str = "f64"; }

        use std::fs::File;
        use std::io::prelude::*;
        fn main() {
            File::create("../out/everywhere.d.ts").expect("Couldn't create .d.ts file")
                        .write_all(
                            (String::new() $(
                                + "export function " + stringify!($fname) + "(" + &vec![$(
                                    stringify!($arg).to_owned() + ": " + <$arg_t as Js>::JS
                                ),*].join(", ") + "): " + <$t as Js>::JS + ";\n"
                            )*)
                            .as_bytes()
                        ).expect("Couldn't write .d.ts file");
        }
    }
}

include!("./index.rs");
