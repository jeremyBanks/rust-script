use {
    proc_macro2::Span,
    std::borrow::{Borrow, BorrowMut},
};

pub trait SpanExt: Borrow<Span> + BorrowMut<Span> {
    fn lo(&self) -> usize;
    fn hi(&self) -> usize;
}

impl SpanExt for Span {
    fn lo(&self) -> usize {
        let span: Span = *self;
        let (_discriminant, (lo, _hi)): (u32, (u32, u32)) = unsafe { std::mem::transmute(span) };
        lo.try_into().unwrap()
    }

    fn hi(&self) -> usize {
        let span: Span = *self;
        let (_discriminant, (_lo, hi)): (u32, (u32, u32)) = unsafe { std::mem::transmute(span) };
        hi.try_into().unwrap()
    }
}
