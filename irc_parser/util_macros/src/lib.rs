use from_irc_string::{command_args_struct::impl_from_irc_string_for_command_args_struct, commands_enum::impl_from_irc_string_for_commands_enum};
use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

mod from_irc_string;

#[proc_macro_derive(FromIRCString, attributes(command_name))]
pub fn derive_from_irc_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    match input.data {
        syn::Data::Struct(struct_data) => {
            impl_from_irc_string_for_command_args_struct(ident, struct_data)
        },
        syn::Data::Enum(enum_data) if ident.to_string().contains("Command") => {
            impl_from_irc_string_for_commands_enum(ident, enum_data)
        }
        _ => syn::Error::new(
            input.span(),
            "Can only apply to structs or \"Command\" enums",
        )
        .to_compile_error()
        .into(),
    }
}
