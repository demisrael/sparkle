cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        pub mod error;
        pub mod imports;
        pub mod limits;
        pub mod params;
        pub mod result;
        pub mod service;

        pub use service::*;
    }
}
