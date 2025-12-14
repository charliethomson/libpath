use std::{path::PathBuf, time::SystemTime};

use libproduct::{product_name, ProductName};
use tracing::{instrument, Level};

product_name!(with base "dev.thmsn.unspecified" as DEFAULT_PRODUCT_NAME);

trait MkdirIfNotExists {
    fn mkdir_if_not_exists(self) -> Self;
}
impl MkdirIfNotExists for PathBuf {
    fn mkdir_if_not_exists(self) -> Self {
        if self.exists() {
            return self;
        }
        std::fs::create_dir_all(&self).expect("Failed to create directory");
        self
    }
}

#[must_use]
fn epoch() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Why are you in the past?")
        .as_secs()
}

#[must_use]
#[instrument(level = Level::DEBUG, ret)]
/// # Panics
///
/// Panics if `dirs` is unable to locate a [data_local_dir](https://docs.rs/dirs/latest/dirs/fn.data_local_dir.html)
pub fn data_root() -> PathBuf {
    let product_name =
        ProductName::global().map_or(DEFAULT_PRODUCT_NAME.to_string(), |name| name.to_string());
    dirs::data_local_dir()
        .expect("cant find data local dir")
        .join(&product_name)
        .mkdir_if_not_exists()
}

#[must_use]
#[instrument(level = Level::DEBUG, ret)]
pub fn configs_root() -> PathBuf {
    data_root().join("configs").mkdir_if_not_exists()
}

#[must_use]
#[instrument(level = Level::DEBUG, ret)]
pub fn config_path(module: &str) -> PathBuf {
    configs_root()
        .mkdir_if_not_exists()
        .join(format!("{module}.toml"))
}

#[must_use]
#[instrument(level = Level::DEBUG, ret)]
pub fn logs_root() -> PathBuf {
    data_root().join("logs").mkdir_if_not_exists()
}

#[must_use]
#[instrument(level = Level::DEBUG, ret)]
pub fn log_path() -> PathBuf {
    logs_root()
        .mkdir_if_not_exists()
        .join(format!("log_{}.json", epoch()))
}
