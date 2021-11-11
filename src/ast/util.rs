use pest::Span;
use std::marker::PhantomData;

pub fn span_into_str(span: Span) -> &str {
    span.as_str()
}

pub fn span_into_phantomdata(_: Span) -> PhantomData<()> {
    PhantomData
}
