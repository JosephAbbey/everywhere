macro_rules! everywhere {
    (
        $(pub fn $fname:ident($($arg:ident: $arg_t:ty),*) -> $t:ty $body:block)*
    ) => {
        use std::mem;
        use std::ptr;

        pub trait Wasm { type Wasm; fn to_wasm(self) -> Self::Wasm; unsafe fn from_wasm(s: Self::Wasm) -> Box<Self>; }
        impl Wasm for String {
            type Wasm = *mut String;
            fn to_wasm(self) -> Self::Wasm { let s = Box::new(self.into()); Box::into_raw(s) }
            unsafe fn from_wasm(s: Self::Wasm) -> Box<Self> { Box::new(mem::transmute::<_, &String>(s).to_string()) }
        }
        impl Wasm for bool { type Wasm = bool; fn to_wasm(self) -> Self::Wasm { self } unsafe fn from_wasm(s: Self::Wasm) -> Box<Self> { Box::new(s) } }
        impl Wasm for f64 { type Wasm = f64; fn to_wasm(self) -> Self::Wasm { self } unsafe fn from_wasm(s: Self::Wasm) -> Box<Self> { Box::new(s) } }

        $(
            #[no_mangle]
            pub unsafe extern fn $fname($($arg: <$arg_t as Wasm>::Wasm),*) -> <$t as Wasm>::Wasm {
                $(let $arg = *<$arg_t as Wasm>::from_wasm($arg);)*
                $body.to_wasm()
            }
        )*

        #[no_mangle]
        pub unsafe extern fn wasm_string_new(len: u32) -> *mut String {
            let mut s = Box::new(String::new());
            for _ in 0..len
                { s.push_str("\0"); }
            
            Box::into_raw(s)
        }

        #[no_mangle]
        pub unsafe extern fn wasm_string_drop(s: *mut String) {
            let s = Box::from_raw(s);
            drop(s);
        }

        #[no_mangle]
        pub unsafe extern fn wasm_string_get_len(s: *mut String) -> u32 {
            mem::transmute::<_, &mut String>(s).len() as u32
        }

        #[no_mangle]
        pub unsafe extern fn wasm_string_get_byte(s: *mut String, index: u32) -> u8 {
            ptr::read(mem::transmute::<_, &mut String>(s).as_ptr().offset(index as isize))
        }

        #[no_mangle]
        pub unsafe extern fn wasm_string_set_byte(s: *mut String, index: u32, value: u8) {
            let bytes = mem::transmute::<_, &mut String>(s).as_ptr();
            ptr::write(mem::transmute::<_, *mut u8>(bytes).offset(index as isize), value)
        }
    };
}

include!("./index.rs");
