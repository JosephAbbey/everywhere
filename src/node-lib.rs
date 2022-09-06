macro_rules! everywhere {
    (@step $idx:expr, $cx:expr,) => {};
    (@step $idx:expr, $cx:expr, $arg:ident: $arg_t:ty, $($a:ident: $a_t:ty,)*) => {
        let $arg: $arg_t = $cx.argument::<<$arg_t as Js>::Js>($idx)?.value(&mut $cx);
        everywhere!(@step $idx + 1i32, $cx, $($a: $a_t,)*);
    };
    (
        $(pub fn $fname:ident($($arg:ident: $arg_t:ty),*) -> $t:ty $body:block)*
    ) => {
        $(pub fn $fname($($arg: $arg_t),*) -> $t $body)*

        use neon::prelude::*;
        use neon::handle::Managed;
        pub trait Js { type Js: Managed; fn to_js(self, cx: FunctionContext) -> JsResult<Self::Js>; }
        impl Js for String { type Js = JsString; fn to_js(self, mut cx: FunctionContext) -> JsResult<Self::Js> { Ok(cx.string(self)) } }
        impl Js for bool { type Js = JsBoolean; fn to_js(self, mut cx: FunctionContext) -> JsResult<Self::Js> { Ok(cx.boolean(self)) } }
        impl Js for f64 { type Js = JsNumber; fn to_js(self, mut cx: FunctionContext) -> JsResult<Self::Js> { Ok(cx.number(self)) } }

        mod node {
            use crate as lib;
            use crate::Js;
            use neon::prelude::*;
            $(pub fn $fname(mut cx: FunctionContext) -> JsResult<<$t as Js>::Js> {
                everywhere!(@step 0i32, cx, $($arg: $arg_t,)*);
                lib::$fname($($arg),*).to_js(cx)
            })*
        }

        #[neon::main]
        fn node_main(mut cx: ModuleContext) -> NeonResult<()> {
            $(cx.export_function(stringify!($fname), node::$fname)?;)*
            Ok(())
        }
    };
}

include!("./index.rs");
