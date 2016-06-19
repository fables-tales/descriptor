#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::{TokenTree, Generics};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use rustc_plugin::Registry;
use syntax::util::small_vector::SmallVector;

fn expand_descriptor(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {

    let function_name = cx.ident_of("main");
    let output = cx.ty_infer(sp);
    let empty_body = cx.block(sp, vec![], None);

    MacEager::items(
        SmallVector.one(
            cx.item_fn(
                sp,
                function_name,
                vec![],
                output,
                empty_body
                )
            )
        )
    );
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("descriptor", expand_descriptor);
}
