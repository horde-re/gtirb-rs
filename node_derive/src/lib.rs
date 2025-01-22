use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Node)]
pub fn derive_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let node_type = match name.to_string().as_str() {
        "CodeBlock" => quote! { NodeType::CodeBlock },
        "ProxyBlock" => quote! { NodeType::ProxyBlock },
        "DataBlock" => quote! { NodeType::DataBlock },
        "Ir" => quote! { NodeType::Ir },
        "Module" => quote! { NodeType::Module },
        "Section" => quote! { NodeType::Section },
        "Symbol" => quote! { NodeType::Symbol },
        "ByteInterval" => quote! { NodeType::ByteInterval },
        _ => quote! { NodeType::Node },
    };

    let expanded = quote! {
        use super::node::*;
        impl Node for #name {
            type UUID = Uuid;

            fn get_uuid(&self) -> &Self::UUID {
                &self.uuid
            }

            fn get_type(&self) -> NodeType {
                #node_type
            }
        }
    };

    TokenStream::from(expanded)
}
