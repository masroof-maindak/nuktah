#[macro_export]
macro_rules! convert_across_err {
    ($src:ty, $dst:ty, $variant:ident) => {
        impl From<$src> for $dst {
            fn from(err: $src) -> $dst {
                <$dst>::$variant(err)
            }
        }
    };
}
