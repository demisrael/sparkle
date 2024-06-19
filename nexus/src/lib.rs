cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {

        pub mod context;
        pub mod error;
        pub mod imports;
        pub mod operations;
        pub mod optypes;
        #[allow(clippy::module_inception)]
        pub mod nexus;
        pub mod event;
        pub mod analyzer;
        pub mod result;

        pub mod prelude {
            pub use crate::nexus::Nexus;
            pub use crate::analyzer::Analyzer;
            pub use crate::context::ContextT;
        }
    }
}
