pub fn type_from_inside_option(field: &syn::Field) -> Option<&syn::Type> {
    let path = if let syn::Type::Path(type_path) = &field.ty {
        if type_path.qself.is_some() {
            return None;
        }
        &type_path.path
    } else {
        return None;
    };
    let segment = path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    let generic_params =
        if let syn::PathArguments::AngleBracketed(generic_params) = &segment.arguments {
            generic_params
        } else {
            return None;
        };
    if let syn::GenericArgument::Type(ty) = generic_params.args.first()? {
        Some(ty)
    } else {
        None
    }
}
