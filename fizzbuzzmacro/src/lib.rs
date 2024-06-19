use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};
use quote::quote;

#[proc_macro]
pub fn fizzbuzz(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match helper(input.into()) {
        Ok(x) => {
            // eprintln!("{x:#?}");
            x.into()
        }
        Err(diag) => diag.emit_as_expr_tokens().into(),
    }
}

fn helper(input: TokenStream) -> Result<TokenStream, Diagnostic> {
    // println!("{input:#?}");
    let mut iter = input.into_iter();
    let loopvar = match iter.next() {
        Some(TokenTree::Ident(x)) => x,
        Some(x) => return Err(x.span().error(format!("{x} should be an identifier"))),
        None => return Err(Span::call_site().error("missing identifiers")),
    };
    let values = iter
        .collect::<Vec<TokenTree>>()
        .chunks(2)
        .map(|x| {
            if let TokenTree::Punct(_) = &x[0] {
            } else {
                return Err(Span::call_site().error("Missing punctuation"));
            }
            match &x[1] {
                TokenTree::Group(x) => {
                    let mut iter = x.stream().into_iter();
                    if let (Some(a), Some(b)) = (iter.next().clone(), iter.nth(1).clone()) {
                        Ok((a, b))
                    } else {
                        Err(x.span().error("Missing items"))
                    }
                }
                x => Err(x.span().error("Wrong type")),
            }
        })
        .collect::<Result<Vec<(TokenTree, TokenTree)>, Diagnostic>>()?;
    // println!("{values:#?}");
    Ok([
        TokenTree::Ident(Ident::new("match", Span::call_site())),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            values
                .iter()
                .flat_map(|x| {
                    [
                        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        TokenTree::Ident(loopvar.clone()),
                        TokenTree::Punct(Punct::new('%', Spacing::Alone)),
                        x.1.clone(),
                    ]
                })
                .skip(1)
                .collect::<TokenStream>(),
        )),
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            (1..1 << values.len())
                .rev()
                .flat_map(|x| {
                    [
                        TokenTree::Group(Group::new(
                            Delimiter::Parenthesis,
                            (0..values.len())
                                .flat_map(|i| {
                                    [
                                        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                                        if (x >> i & 1) == 1 {
                                            TokenTree::Literal(Literal::i32_unsuffixed(0))
                                        } else {
                                            TokenTree::Ident(Ident::new("_", Span::call_site()))
                                        },
                                    ]
                                })
                                .skip(1)
                                .collect::<TokenStream>(),
                        )),
                        TokenTree::Punct(Punct::new('=', Spacing::Joint)),
                        TokenTree::Punct(Punct::new('>', Spacing::Alone)),
                        TokenTree::Group(Group::new(
                            Delimiter::Bracket,
                            values
                                .iter()
                                .enumerate()
                                .filter(|(i, _)| x >> i & 1 == 1)
                                .map(|x| x.1 .0.clone())
                                .flat_map(|a| {
                                    [TokenTree::Punct(Punct::new(',', Spacing::Alone)), a]
                                })
                                .skip(1)
                                .collect::<TokenStream>(),
                        )),
                    ]
                    .into_iter()
                    .chain(quote! {
                    .into_iter().collect::<String>(),
                                      })
                })
                .chain([
                    TokenTree::Group(Group::new(
                        Delimiter::Parenthesis,
                        (0..values.len())
                            .flat_map(|_| {
                                [
                                    TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                                    TokenTree::Ident(Ident::new("_", Span::call_site())),
                                ]
                            })
                            .skip(1)
                            .collect::<TokenStream>(),
                    )),
                    TokenTree::Punct(Punct::new('=', Spacing::Joint)),
                    TokenTree::Punct(Punct::new('>', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("format", Span::call_site())),
                    TokenTree::Punct(Punct::new('!', Spacing::Alone)),
                    TokenTree::Group(Group::new(
                        Delimiter::Parenthesis,
                        [
                            TokenTree::Literal(Literal::string("{}")),
                            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                            TokenTree::Ident(loopvar),
                        ]
                        .into_iter()
                        .collect::<TokenStream>(),
                    )),
                ])
                .collect::<TokenStream>(),
        )),
    ]
    .into_iter()
    .collect::<TokenStream>())
}
