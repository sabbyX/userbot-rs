
#[macro_export]
/// Macro to implement [`ToString`] for kantex-rs items
macro_rules! implement_to_string {
    ($( $types:ty ),*) => (
        $(
            impl ToString for $types {
                fn to_string(&self) -> String {
                    self.stringify()
                }
            }
        )*
    )
}
