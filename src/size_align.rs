use syn::{Error, LitInt, Result, DeriveInput, Token};
use proc_macro::TokenStream;
use quote::quote;

pub struct SizeAlign {
    pub size: usize,
    pub align: usize,
}

/// Custom way to parse and type-check proc macro.
/// #[proc_macro(attr_1 : value_1 , attr_2 : value_2)].
/// 
/// This attribute is used primarily during development 
/// and testing phases to enforce specific memory
/// layout requirements for structs.
impl syn::parse::Parse for SizeAlign {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut size = None;
        let mut align = None;

        while !input.is_empty() {
            let field_name: syn::Ident = input.parse()?;                                        // Parse the first token as an identifier (field name)
            input.parse::<Token![:]>()?;                                                        // Expect a colon (:) after the identifier

            match field_name.to_string().as_str() {
                "size" => {
                    let value: LitInt = input.parse()?;                                         // Parse the next token as an identifier (field name)
                    size = Some(value.base10_parse::<usize>()?);                                // Expect a colon (:) after the identifier
                }
                "align" => {
                    let value: LitInt = input.parse()?;                                         // Parse the literal integer value after "size"
                    align = Some(value.base10_parse::<usize>()?);                               // Parse the literal integer value after "align"
                }
                _ => return Err(Error::new(field_name.span(), "Unexpected field")),     // Return an error for unexpected field names
            }

            if input.is_empty() {
                break;
            }

            input.parse::<Token![,]>()?;                                                        // Expect a comma (,) after each parsed field
        }

        Ok(SizeAlign {                                                                          // Ensure both size and align are provided, otherwise return an error
            size: size.ok_or_else(|| Error::new(input.span(), "Missing field `size`"))?,
            align: align.ok_or_else(|| Error::new(input.span(), "Missing field `align`"))?,
        })
    }
}

/// Handle assertions.
/// TODO: Allow nested struct alignment check.
pub fn assert_align_size_impl(size: usize, align: usize, input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    let generated_code = quote! {
        #input                                                                  // will be replaced with the actual struct passed to the macro.

        const _: () = {
            const _: [(); ::core::mem::size_of::<#name>()] = [(); #size];       // Size assertion
            const _: [(); ::core::mem::align_of::<#name>()] = [(); #align];     // Alignment assertion for the struct itself
        };
    };

    generated_code.into()
}
