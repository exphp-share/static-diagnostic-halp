pub use mac_impl::attr;
pub use static_assertions;

#[macro_export]
macro_rules! check_smn {
    () => { $crate::static_assertions::const_assert_eq!(1, 2); }
}
