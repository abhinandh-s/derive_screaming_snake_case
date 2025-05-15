use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

fn to_screaming_snake_case(input: &str) -> String {
    let mut result = String::new();
    for (i, ch) in input.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                result.push('_');
            }
            result.push(ch);
        } else {
            result.push(ch.to_ascii_uppercase());
        }
    }
    result
}

/// Implements the `Display` trait for enums by converting each variant name to SCREAMING_SNAKE_CASE.
///
/// # Example
/// ```
/// use derive_screaming_snake_case::Display;
///
/// #[derive(Display)]
/// enum Status {
///     Ok,
///     NotFound,
///     InternalServerError,
/// }
///
/// assert_eq!(Status::Ok.to_string(), "OK");
/// assert_eq!(Status::NotFound.to_string(), "NOT_FOUND");
/// assert_eq!(Status::InternalServerError.to_string(), "INTERNAL_SERVER_ERROR");
/// ```
///
/// # Panics
/// This macro will cause a **compilation error** if applied to anything other than a unit enum.
#[proc_macro_derive(Display)]
pub fn display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = parse_macro_input!(input as DeriveInput);
    let name = source.ident;
    let variants = match source.data {
        Data::Enum(data_enum) => data_enum.variants,
        _ => {
            return syn::Error::new_spanned(name, "#[derive(Display)] can only be used on enums")
                .to_compile_error()
                .into();
        }
    };

    let arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        match variant.fields {
            syn::Fields::Unit => {
                let screaming = to_screaming_snake_case(&ident.to_string());
                quote! {
                    Self::#ident => write!(f, #screaming),
                }
            }
            _ => syn::Error::new_spanned(
                variant,
                "#[derive(Display)] only supports unit variants (no data fields)",
            )
                .to_compile_error(),
        }
    });
    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

                match self {
                    #(#arms)*
                }
            }
        }
    };
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_screaming_snake_case_test() {
        let input = "ThisWillGetScreamingSnakeCase";
        let re = to_screaming_snake_case(input);
        assert_eq!(re, "THIS_WILL_GET_SCREAMING_SNAKE_CASE".to_owned())
    }
}
