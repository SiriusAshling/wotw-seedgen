use quote::{quote, ToTokens};

pub fn parse_token_impl(mut input: syn::ItemEnum) -> proc_macro::TokenStream {
    let ident = input.ident.clone();

    input
        .attrs
        .push(syn::parse_quote!(#[logos(source = str, error = wotw_seedgen_parse::Error)]));

    let mut tokens = logos_codegen::generate(input.into_token_stream());
    tokens.extend(quote! {
        impl<'source> wotw_seedgen_parse::ParseToken<'source> for #ident {}
    });

    tokens.into()
}
