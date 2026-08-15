use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, parse_macro_input};

#[proc_macro_derive(Renderable)]
pub fn derive_renderable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Renderable requires named fields"),
        },
        _ => panic!("Renderable can only be derived for structs"),
    };

    let field_names_str: Vec<LitStr> = fields
        .iter()
        .map(|field| {
            // Get the identifier (handles named fields like `id: u32`)
            let ident = field.ident.as_ref().expect("Expected a named field");
            // Convert the identifier into a syn string literal
            LitStr::new(&ident.to_string(), ident.span())
        })
        .collect();

    // 2. Extract types
    let field_types: Vec<&syn::Type> = fields
        .iter()
        .map(|field| &field.ty)
        .collect();

    // for field in &fields {
    //     let field_name = field.ident.as_ref().unwrap();
    //     let field_type = &field.ty;

    //     println!("field: {field_name:#?}, type: {field_type:#?}");
    // }

    let expanded = quote! {
        impl ::in_form::Renderable for #name {
            fn get_component() -> ::in_form::Component {
                ::in_form::Component::FieldStruct(vec![
                    #( 
                        ::in_form::NamedComponent {
                            name: #field_names_str,
                            comp: #field_types ::get_component(),
                        },
                    )*
                ])
            }
        }
    };

    TokenStream::from(expanded)
}
