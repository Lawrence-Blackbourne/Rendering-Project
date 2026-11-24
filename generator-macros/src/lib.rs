/// Limitations:
/// Can only handle impl typename {} and not impl ... for typename {}

mod token_handler;

use proc_macro::TokenStream;

#[proc_macro]
pub fn create_generator(input: TokenStream) -> TokenStream {
    todo!()
}