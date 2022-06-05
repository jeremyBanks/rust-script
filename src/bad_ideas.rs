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
        let (discriminant, (mut lo, mut hi)): (u32, (u32, u32)) =
            unsafe { std::mem::transmute(span) };
        if lo > hi {
            // yikes
            std::mem::swap(&mut lo, &mut hi);
        }
        if discriminant > 1 {
            panic!("this was a bad idea");
        }
        lo.try_into().unwrap()
    }

    fn hi(&self) -> usize {
        let span: Span = *self;
        let (discriminant, (mut lo, mut hi)): (u32, (u32, u32)) =
            unsafe { std::mem::transmute(span) };
        if lo > hi {
            // yikes
            std::mem::swap(&mut lo, &mut hi);
        }
        if discriminant > 1 {
            panic!("this was a bad idea");
        }
        hi.try_into().unwrap()
    }
}
