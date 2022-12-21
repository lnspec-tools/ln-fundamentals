//! Derive crate implement the procedural macros
//! to implement `FromWire` and `ToWire` in a
//! struct.
use gen_impl::{gen_from_wire_impl, gen_to_wire_impl};
use kproc_parser::kparser::DummyTracer;
use kproc_parser::kproc_macros::KTokenStream;
use kproc_parser::proc_macro::TokenStream as TokenStream2;
use kproc_parser::rust::ast::RustAST;
use kproc_parser::rust::rust_struct::parse_struct;
use proc_macro::TokenStream;

mod gen_impl;

#[proc_macro_derive(EncodeWire)]
pub fn impl_to_wire(tokens: TokenStream) -> TokenStream {
    let inputv2 = TokenStream2::from(tokens.clone());
    let mut stream = KTokenStream::new(&inputv2);
    let tracer = DummyTracer {};
    let RustAST::Struct(ast) = parse_struct(&mut stream, &tracer);
    let to_wire_impl = gen_to_wire_impl(&ast);
    to_wire_impl.parse().unwrap()
}

#[proc_macro_derive(DecodeWire)]
pub fn impl_from_wire(tokens: TokenStream) -> TokenStream {
    let inputv2 = TokenStream2::from(tokens.clone());
    let mut stream = KTokenStream::new(&inputv2);
    let tracer = DummyTracer {};
    let RustAST::Struct(ast) = parse_struct(&mut stream, &tracer);
    let from_wire_impl = gen_from_wire_impl(&ast);
    from_wire_impl.parse().unwrap()
}
