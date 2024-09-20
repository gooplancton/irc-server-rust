use proc_macro::TokenStream;
use quote::quote;
use syn::DataStruct;

use super::utils::type_from_inside_option;

pub fn impl_from_irc_string_for_command_args_struct(
    ident: &syn::Ident,
    struct_data: DataStruct,
) -> TokenStream {
    let field_idents = struct_data.fields.iter().map(|field| field.ident.as_ref());
    let field_assignments = struct_data.fields.iter().map(|field| {
        let field_ident = field.ident.as_ref();
        let field_type = &field.ty;

        if let Some(option_inner_type) = type_from_inside_option(field) {
            quote! {
                let #field_ident = if let Some(arg_string) = args.next() {
                    Some(arg_string.parse::<#option_inner_type>()?)
                } else {
                    None
                };
            }
        } else {
            quote! {
                let arg_string = args.next().ok_or(anyhow::anyhow!("Not enough arguments"))?;
                let #field_ident = arg_string.parse::<#field_type>()?;
            }
        }
    });

    let expanded = quote! {
        impl irc_parser::FromIRCString for #ident {
            fn from_irc_string(arg_string: &str) -> anyhow::Result<Self> {
                let args: Vec<&str> = match arg_string.split_once(" :") {
                    None => arg_string.split(' ').collect(),
                    Some((first_args, last_arg)) => {
                        let mut args: Vec<&str> = first_args.split(' ').collect();
                        args.push(last_arg);

                        args
                    },
                };

                let mut args = args.into_iter();

                #(#field_assignments)*

                Ok(Self {
                    #(#field_idents),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
