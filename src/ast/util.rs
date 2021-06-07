use pest::Span;
use std::marker::PhantomData;

pub fn span_into_str(span: Span) -> &str {
    span.as_str()
}

pub fn string_to_static_str(s: &str) -> &'static str {
    Box::leak(s.to_owned().into_boxed_str())
}

pub fn span_into_phantomdata(_: Span) -> PhantomData<()> {
    PhantomData
}
