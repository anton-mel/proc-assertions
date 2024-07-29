use syn::{
    self, parenthesized, 
    parse::{Parse, ParseStream, ParseBuffer}, 
    Ident, LitStr, Result, Token};

pub struct WhitelistArgs {
    pub struct_name: String,
    pub values: Vec<String>,
}

impl Parse for WhitelistArgs {
    fn parse (input: ParseStream) -> Result<Self> {
        // #[mutates(struct_name: (field1, field2, ...))]
        let struct_name: Ident = input.parse()?;
        let struct_name = struct_name.to_string();
        // Expect a column before func whitelist.
        input.parse::<Token![:]>()?;
        // Parse the field_names.
        let content: ParseBuffer;
        parenthesized!(content in input);
        let mut values: Vec<String> = Vec::new();
        while !content.is_empty() {
            let value: LitStr = content.parse()?;
            values.push(value.value());
            // Check for more values.
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            } else {
                break;
            }
        } 

        Ok(WhitelistArgs {
            struct_name,
            values
        })
    }
}
