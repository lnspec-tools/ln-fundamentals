//! Helper module to generate the implementation
//! for the struct.
use kproc_parser::rust::ast_nodes::StructToken;

pub(crate) fn gen_to_wire_impl(ast_struct: &StructToken) -> String {
    let mut code = format!("impl ToWire for {} {{\n", ast_struct.name);
    code += get_to_wire_fn(ast_struct).as_str();
    code += "}\n";
    code
}

pub(crate) fn get_to_wire_fn(ast_struct: &StructToken) -> String {
    let mut code = "fn to_wire<W: Write>(&self, buff: &mut W) -> Result<(), IOError> {".to_owned();
    for field in &ast_struct.fields {
        code += format!("   self.{}.to_wire(buff)?;\n", field.name).as_str();
    }
    code += "Ok(())\n }\n";
    code
}

pub(crate) fn gen_from_wire_impl(ast_struct: &StructToken) -> String {
    let mut code = format!("impl FromWire for {} {{\n", ast_struct.name);
    code += get_from_wire_fn(ast_struct).as_str();
    code += "}\n";
    code
}

pub(crate) fn get_from_wire_fn(ast_struct: &StructToken) -> String {
    let mut code = "fn from_wire<R: Read>(buff: &mut R) -> Self {".to_owned();
    for field in &ast_struct.fields {
        code += format!("let {} = {}::from_wire(buff);\n", field.name, field.ty).as_str();
    }
    code += "todo!()";
    code += "}\n";
    code
}
