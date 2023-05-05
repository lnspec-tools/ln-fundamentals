//! Helper module to generate the implementation
//! for the struct.
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::rust::ast_nodes::{AttrToken, StructToken};

pub(crate) fn gen_to_wire_impl(ast_struct: &StructToken) -> TokenStream {
    let mut code = format!("impl ToWire for {} {{\n", ast_struct.name);
    code += &get_to_wire_fn(ast_struct);
    code += "}\n";
    // SAFETY: safe to unwrap this because the code generated should be valid rust
    code.parse().unwrap()
}

pub(crate) fn get_to_wire_fn(ast_struct: &StructToken) -> String {
    let mut code = "fn to_wire<W: Write>(&self, buff: &mut W) -> std::io::Result<()> {".to_owned();
    for field in &ast_struct.fields {
        let tmp = if let Some(AttrToken::Attr(value)) = field.attrs.get("msg_type") {
            let attr = value.to_owned().value.unwrap();
            format!(" ({} as u16).to_wire(buff)?;\n", attr)
        } else {
            format!("   self.{}.to_wire(buff)?;\n", field.identifier)
        };
        code += tmp.as_str();
    }
    code += "Ok(())\n }\n";
    code
}

pub(crate) fn gen_from_wire_impl(ast_struct: &StructToken) -> TokenStream {
    let mut code = format!("impl FromWire for {} {{\n", ast_struct.name);
    code += get_from_wire_fn(ast_struct).as_str();
    code += "}\n";
    // SAFETY: this is safe because the code generated should be valid rust
    code.parse().unwrap()
}

pub(crate) fn get_from_wire_fn(ast_struct: &StructToken) -> String {
    let mut code = "fn from_wire<R: Read>(reader: &mut R) -> std::io::Result<Self> {".to_owned();
    let mut fields = Vec::<String>::new();
    for field in &ast_struct.fields {
        let field_name = &field.identifier;
        code += &format!("let {field_name} = {}::from_wire(reader)?;\n", field.ty);
        fields.push(field.identifier.to_string());
    }
    code += &format!("Ok(Self::_new({}))\n", fields.join(","));
    code += "}\n";
    code
}

pub(crate) fn generate_from_write_new_method(ast_struct: &StructToken) -> TokenStream {
    let struct_name = &ast_struct.name;
    let fields = ast_struct
        .fields
        .iter()
        .map(|field| field.identifier.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let params = ast_struct
        .fields
        .iter()
        .map(|field| format!("{}: {}", field.identifier, field.ty))
        .collect::<Vec<String>>()
        .join(",");
    format!(
        "
    impl {struct_name} {{
         fn _new({params}) -> Self {{
              Self{{ {fields} }}
         }}
    }}
"
    )
    .parse()
    .unwrap()
}
