use from_irc_string::{
    command_args_struct::impl_from_irc_string_for_command_args_struct,
    commands_enum::impl_from_irc_string_for_commands_enum,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

mod from_irc_string;

#[proc_macro_derive(FromIRCString, attributes(command_name, command_list))]
pub fn derive_from_irc_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let is_commands_enum = input.attrs.iter().any(|attr| match &attr.meta {
        syn::Meta::Path(path) => path.is_ident("command_list"),
        _ => false,
    });

    match input.data {
        syn::Data::Struct(struct_data) => {
            impl_from_irc_string_for_command_args_struct(ident, struct_data)
        }
        syn::Data::Enum(enum_data) if is_commands_enum => {
            impl_from_irc_string_for_commands_enum(ident, enum_data)
        }
        _ => syn::Error::new(
            input.span(),
            "Can only apply to structs or enums tagged with the #command_list attribute",
        )
        .to_compile_error()
        .into(),
    }
}

#[proc_macro_derive(RunCommand)]
pub fn run_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let enum_data = match input.data {
        syn::Data::Enum(enum_data) => enum_data,
        _ => {
            return syn::Error::new(input.span(), "Can only apply to enums")
                .to_compile_error()
                .into();
        }
    };

    let arms = enum_data.variants.into_iter().map(|variant| {
        let variant_ident = &variant.ident;
        quote! {
            Self::#variant_ident(args) => args.run(state, outbox).await,
        }
    });

    let expanded = quote! {
        impl RunCommand for #ident {
            async fn run(
                self,
                state: &crate::internals::ConnectionState,
                outbox: tokio::sync::mpsc::Sender<crate::internals::Message>
            ) -> anyhow::Result<crate::commands::CommandOutput> {
                match self {
                    #(#arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
