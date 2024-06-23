use crate::imports::*;

/// The Db type used for Sparkle stores
pub struct Db {
    inner: DBWithThreadMode<MultiThreaded>,
    _fd_guard: FDGuard,
}

impl Db {
    pub fn new(inner: DBWithThreadMode<MultiThreaded>, fd_guard: FDGuard) -> Self {
        Self {
            inner,
            _fd_guard: fd_guard,
        }
    }
}

impl DerefMut for Db {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Deref for Db {
    type Target = DBWithThreadMode<MultiThreaded>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Deletes an existing Db if it exists
pub fn delete_db(db_dir: PathBuf) {
    if !db_dir.exists() {
        return;
    }
    let options = rocksdb::Options::default();
    let path = db_dir.to_str().unwrap();
    <DBWithThreadMode<MultiThreaded>>::destroy(&options, path)
        .expect("DB is expected to be deletable");
}
