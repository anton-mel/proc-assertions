use syn::{
    parse::{Parse, ParseStream},
    LitStr, Result, Token,
};

pub struct WhitelistArgs {
    pub values: Vec<String>,
}

impl Parse for WhitelistArgs {
    // #[calledby("func1", "func2", ...)]
    fn parse(input: ParseStream) -> Result<Self> {
        let mut values = Vec::new();

        while !input.is_empty() {
            let function_name: LitStr = input.parse()?;
            values.push(function_name.value());

            if !input.is_empty() && input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(WhitelistArgs { values })
    }
}
