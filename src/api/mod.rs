pub mod v2;

#[macro_export]
macro_rules! concat_endpoint {
    ($e:expr) => {
        concat!("https://universalis.app", $e)
    };
}
