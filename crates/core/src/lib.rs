pub use placard_ipc_types as types;

pub mod config {
    use std::path::PathBuf;

    pub fn app_data_dir() -> PathBuf {
        let base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        base.join("placard")
    }

    pub fn library_dir() -> PathBuf {
        app_data_dir().join("library")
    }

    pub fn db_path() -> PathBuf {
        app_data_dir().join("placard.db")
    }
}

pub mod keyring_store {
    use keyring::Entry;

    const SERVICE_NAME: &str = "dev.placard.desktop";

    pub fn store_key(source_id: &str, key: &str) -> anyhow::Result<()> {
        let entry = Entry::new(SERVICE_NAME, source_id)?;
        entry.set_password(key)?;
        Ok(())
    }

    pub fn get_key(source_id: &str) -> anyhow::Result<Option<String>> {
        let entry = Entry::new(SERVICE_NAME, source_id)?;
        match entry.get_password() {
            Ok(pwd) => Ok(Some(pwd)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("Keyring error: {e}")),
        }
    }

    pub fn delete_key(source_id: &str) -> anyhow::Result<()> {
        let entry = Entry::new(SERVICE_NAME, source_id)?;
        match entry.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("Keyring error: {e}")),
        }
    }
}
