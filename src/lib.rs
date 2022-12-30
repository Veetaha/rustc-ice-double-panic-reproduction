#[derive(from_variants::FromVariants)]
pub(crate) enum FooBarity {
    Bar(bool, u8),
}
