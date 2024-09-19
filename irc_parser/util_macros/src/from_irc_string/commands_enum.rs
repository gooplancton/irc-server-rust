use proc_macro::TokenStream;
use quote::quote;
use syn::DataEnum;

pub fn impl_from_irc_string_for_commands_enum(
    ident: &syn::Ident,
    enum_data: DataEnum,
) -> TokenStream {
    let arms = enum_data.variants.into_iter().map(|variant| {
        let variant_ident = &variant.ident;
        let args_type = &variant
            .fields
            .iter()
            .next()
            .expect("missing arg struct in enum variant")
            .ty;

        let command_name = variant
            .attrs
            .into_iter()
            .filter_map(|attr| match attr.meta {
                syn::Meta::NameValue(meta) => {
                    if meta.path.get_ident().map(|ident| ident.to_string())
                        == Some("command_name".to_string())
                    {
                        Some(meta.value)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .next()
            .expect("enum variants need to have the command_name attribute"); // TODO: return proper syn error

        quote! {
            #command_name => {
                let args = <#args_type>::from_irc_string(arg_string).unwrap();
                Ok(#ident::#variant_ident(args))
            }
        }
    });

    let expanded = quote! {
        impl FromIRCString for #ident {

            fn from_irc_string(irc_string: &str) -> anyhow::Result<Command> {
                let irc_string = irc_string.trim_end();
                let space_idx = irc_string.find(' ');
                let (command_name, arg_string) = match space_idx {
                    None => (irc_string, ""),
                    Some(idx) => {
                        let arg_string = irc_string[idx + 1..].trim_end();
                        let command_name = irc_string[..idx].trim_end();
                        (command_name, arg_string)
                    }
                };

                match command_name {
                    #(#arms)*
                    _ => Err(anyhow::anyhow!("unknown command: {}", command_name)),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

