use crate::ToTokens;
use proc_macro2::{Span, TokenStream};

pub trait Spanned {
    fn __span(&self) -> Span;
}

impl Spanned for Span {
    fn __span(&self) -> Span {
        *self
    }
}

impl<T: ?Sized + ToTokens> Spanned for T {
    fn __span(&self) -> Span {
        join_spans(self.into_token_stream())
    }
}

fn join_spans(tokens: TokenStream) -> Span {
    #[cfg(not(needs_invalid_span_workaround))]
    let mut iter = tokens.into_iter().map(|tt| tt.span());

    #[cfg(needs_invalid_span_workaround)]
    let mut iter = tokens.into_iter().filter_map(|tt| {
        let span = tt.span();
        let debug = format!("{:?}", span);
        if debug.ends_with("bytes(0..0)") {
            None
        } else {
            Some(span)
        }
    });

    let first = match iter.next() {
        Some(span) => span,
        None => return Span::call_site(),
    };

    iter.fold(None, |_prev, next| Some(next))
        .and_then(|last| first.join(last))
        .unwrap_or(first)
}
