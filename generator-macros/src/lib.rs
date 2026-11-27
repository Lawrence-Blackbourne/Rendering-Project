/// Limitations:
/// Can only handle impl typename {} and not impl ... for typename {}

mod token_handler;
mod tokens;

#[cfg(not(test))]
use proc_macro::TokenStream;

#[cfg(not(test))]
#[proc_macro]
pub fn create_generator(input: TokenStream) -> TokenStream {
    let mut tokens = token_handler::Items::new(input);
    let _ = tokens.next();
    panic!()
}