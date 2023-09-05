use proc_macro::TokenStream;
use syn::{self, parse_macro_input, Ident, Data, FieldsNamed, TypePath};
use syn::DeriveInput;
use syn::DataStruct;
use quote::{quote, ToTokens};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _ = input;
    let derive_input: DeriveInput = parse_macro_input!(input as DeriveInput);
    eprintln!("HAllo");
    eprintln!("{:#?}", derive_input);

    let bname = format!("{}Builder", derive_input.ident);
    let bident = Ident::new(&bname, derive_input.ident.span());
    let ident = derive_input.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct{
        fields: syn::Fields::Named(FieldsNamed{ref named , ..}),
        ..
    }) = derive_input.data {
        named
    } else {
        unimplemented!()
    };

    println!("{:#?}", fields);

    let attributes = fields.iter().map(|x| {
        eprintln!("KAAS");
        eprintln!("{:#?}", x);

        let field_ident = &x.ident;
        let field_ty: &syn::Type = &x.ty;

        let path_type = if let syn::Type::Path(ref type_path)  = field_ty{
            type_path.path.clone()
        } else {
            unimplemented!()
        };

        let t = quote!{
            #field_ident: Option<#path_type>
        };

        t
    });

    let expanded = quote!{
        pub struct #bident {
            #(#attributes),*
            // executable: Option<String>,
            // args: Option<Vec<String>>,
            // env: Option<Vec<String>>,
            // current_dir: Option<String>,
        }

        impl #ident {
            pub fn builder() -> #bident{
                #bident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }

        impl #bident {
            // #(
            //     #(fields.)

            // )*

            pub fn executable(self: &mut #bident, value: String) -> &mut #bident{
                self
            }

            pub fn args(self: &mut #bident, value: Vec<String>) -> &mut #bident{
                self
            }

            pub fn env(self: &mut #bident, value: Vec<String>) -> &mut #bident{
                self
            }

            pub fn current_dir(self: &mut #bident, value: String) -> &mut #bident{
                self
            }
        }
    };

    expanded.into()
}
