use std::{
    panic::{
        UnwindSafe,
        RefUnwindSafe,
    },
    path::Path,
};

use crate::error::Error;

struct Inner {
    db: sled::Db,
}

impl UnwindSafe for Inner {}
impl RefUnwindSafe for Inner {}

/**
A database instance.
*/
pub struct Store {
    inner: Inner,
}

impl Store {
    /**
    Create or open a store at the given location.
    */
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let db = sled::Db::start_default(path).map_err(Error::fail)?;

        Ok(Store {
            inner: Inner { db },
        })
    }

    /**
    Close a store.
    */
    pub fn close(self) -> Result<(), Error> {
        self.inner.db.flush().map_err(Error::fail)?;

        Ok(())
    }
}