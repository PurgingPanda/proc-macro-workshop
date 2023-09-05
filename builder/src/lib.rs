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

    let attributes_for_builder = fields.iter().map(|x| {
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

    let values_for_builder  = fields.iter().map(|x| {
        let field_ident = &x.ident;
        let field_ty: &syn::Type = &x.ty;

        let path_type = if let syn::Type::Path(ref type_path)  = field_ty{
            type_path.path.clone()
        } else {
            unimplemented!()
        };

        let t = quote!{
            #field_ident: None 
        };

        t
    });

    let methods  = fields.iter().map(|x| {
        let field_ident = &x.ident;
        let field_ty: &syn::Type = &x.ty;

        let path_type = if let syn::Type::Path(ref type_path)  = field_ty{
            type_path.path.clone()
        } else {
            unimplemented!()
        };

        let t = quote!{
            pub fn #field_ident(self: &mut #bident, value: #path_type) -> &mut #bident {
                self.#field_ident = Some(value);
                self
            }
        };

        t
    });

    let expanded = quote!{
        pub struct #bident {
            #(#attributes_for_builder),*
        }

        impl #ident {
            pub fn builder() -> #bident{
                #bident {
                    #(#values_for_builder),*
                }
            }
        }

        use std::error;
        // use syn::token::Box;
        use std::boxed::Box;

        impl #bident {
            #(#methods)*

            pub fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
                // CHeck if all fields are set
                // If so return Command
                let executable_value : String = self.executable.clone().ok_or_else(|| Box::<dyn std::error::Error>::from("Executable not set"))?;
                self.executable = None;

                Ok(#ident {
                    executable: executable_value,
                    args: vec![],
                    env: vec![],
                    current_dir: "".into()
                })
            }
        }
    };

    expanded.into()
}
