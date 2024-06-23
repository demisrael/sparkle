//! # Database
//!
//! This is a replicate of kaspa-database crate with
//! modifications for use in the Sparkle project.
//!

cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {

        mod access;
        mod cache;
        mod connection;
        mod db;
        mod error;
        mod result;
        mod item;
        mod key;
        mod writer;

        pub mod imports;
        pub mod registry;
        mod set_access;
        pub mod utils;

        pub mod prelude {
            use crate::*;

            pub use access::CachedDbAccess;
            pub use cache::{Cache, CachePolicy};
            pub use item::{CachedDbItem, CachedDbSetItem};
            pub use key::DbKey;
            pub use set_access::{CachedDbSetAccess, DbSetAccess, ReadLock};
            pub use writer::{BatchDbWriter, DbWriter, DirectDbWriter, DirectWriter, MemoryWriter};
            pub use db::{delete_db, Db};
            pub use connection::ConnBuilder;
            pub use error::{StoreError};
            pub use result::{StoreResult, StoreResultEmptyTuple, StoreResultExtensions};
        }

    }
}
