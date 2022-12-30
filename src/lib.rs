#[derive(from_variants::FromVariants)]
pub(crate) enum FooBarity {
    Foo(u32),
    Bar(bool, u8),
}
