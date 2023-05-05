//! Derive crate implement the procedural macros
//! to implement `FromWire` and `ToWire` in a
//! struct.
use proc_macro::TokenStream;

use kproc_parser::proc_macro::TokenStream as TokenStream2;
use kproc_parser::rust::kparser::RustParser;

mod gen_impl;
use gen_impl::{gen_from_wire_impl, gen_to_wire_impl, generate_from_write_new_method};

#[proc_macro_derive(EncodeWire, attributes(msg_type))]
pub fn impl_to_wire(tokens: TokenStream) -> TokenStream {
    let mut inputv2 = TokenStream2::from(tokens.clone());
    let parser = RustParser::new();
    let struct_ast = parser.parse_struct(&mut inputv2);
    gen_to_wire_impl(&struct_ast)
}

#[proc_macro_derive(DecodeWire)]
pub fn impl_from_wire(tokens: TokenStream) -> TokenStream {
    let inputv2 = TokenStream2::from(tokens.clone());
    let parser = RustParser::new();
    let struct_ast = parser.parse_struct(&inputv2);
    let mut impl_constructor = generate_from_write_new_method(&struct_ast);
    let from_wire_impl = gen_from_wire_impl(&struct_ast);
    impl_constructor.extend(from_wire_impl);
    impl_constructor
}
