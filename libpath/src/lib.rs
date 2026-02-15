//! Application directory and path utilities.
//!
//! This module provides convenience functions for accessing platform-specific
//! directories with automatic creation and product name prefixing.

use libproduct::{ProductName, product_name};
use std::{path::PathBuf, time::SystemTime};
use tracing::{Level, instrument};

product_name!(with base "dev.thmsn.unspecified" as DEFAULT_PRODUCT_NAME);

/// Extension trait for creating directories if they don't exist.
///
/// This trait provides a convenient chainable method for ensuring a directory
/// exists before using it.
pub trait MkdirIfNotExists {
    /// Creates the directory and all parent directories if they don't exist.
    ///
    /// # Panics
    ///
    /// Panics if directory creation fails due to permission errors or I/O issues.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let path = PathBuf::from("/tmp/my_app/data").mkdir_if_not_exists();
    /// ```
    #[must_use]
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

/// Returns the current Unix epoch timestamp in seconds.
///
/// # Panics
///
/// Panics if the system time is set before the Unix epoch (January 1, 1970).
#[must_use]
fn epoch() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Why are you in the past?")
        .as_secs()
}

pub mod dirs {
    use super::{DEFAULT_PRODUCT_NAME, Level, MkdirIfNotExists, PathBuf, ProductName, instrument};

    macro_rules! wrap_dirs {
        ($name:ident, $original_name:ident, $debug_text:literal) => {
            #[doc = concat!(
                "Returns the ", $debug_text, " for the application.\n\n",
                "This function wraps [`dirs::", stringify!($original_name), "`] and appends ",
                "the product name to create an application-specific directory. ",
                "If the directory doesn't exist, it will be created.\n\n",
                "# Panics\n\n",
                "Panics if the ", $debug_text, " cannot be determined for the current platform.\n\n",
                "# See Also\n\n",
                "[`dirs::", stringify!($original_name), "`](https://docs.rs/dirs/latest/dirs/fn.", stringify!($original_name), ".html)"
            )]
            #[must_use]
            #[instrument(level = Level::DEBUG, ret)]
            pub fn $name() -> PathBuf {
                let product_name = ProductName::global()
                    .map_or(DEFAULT_PRODUCT_NAME.to_string(), |name| name.to_string());
                ::dirs::$original_name()
                    .expect(concat!("cant find ", $debug_text))
                    .join(&product_name)
                    .mkdir_if_not_exists()
            }
        };
        ($original_name:ident, $debug_text:literal) => {
            wrap_dirs!($original_name, $original_name, $debug_text);
        };
    }

    wrap_dirs!(audio_dir, "audio directory");
    wrap_dirs!(cache_dir, "cache directory");
    wrap_dirs!(config_dir, "config directory");
    wrap_dirs!(config_local_dir, "local config directory");
    wrap_dirs!(data_dir, "data directory");
    wrap_dirs!(data_local_dir, "local data directory");
    wrap_dirs!(desktop_dir, "desktop directory");
    wrap_dirs!(document_dir, "document directory");
    wrap_dirs!(download_dir, "download directory");
    wrap_dirs!(executable_dir, "executable directory");
    wrap_dirs!(font_dir, "font directory");
    wrap_dirs!(home_dir, "home directory");
    wrap_dirs!(picture_dir, "picture directory");
    wrap_dirs!(preference_dir, "preference directory");
    wrap_dirs!(public_dir, "public directory");
    wrap_dirs!(runtime_dir, "runtime directory");
    wrap_dirs!(state_dir, "state_dir");
    wrap_dirs!(template_dir, "template_dir");
    wrap_dirs!(video_dir, "video_dir");
}

/// Returns the root directory for application configuration files.
///
/// This creates a `configs` subdirectory within the application's local data directory.
/// The directory is created if it doesn't exist.
///
/// # Examples
///
/// ```ignore
/// let configs = configs_root();
/// // On Linux: ~/.local/share/{product_name}/configs
/// // On macOS: ~/Library/Application Support/{product_name}/configs
/// // On Windows: C:\Users\{user}\AppData\Local\{product_name}\configs
/// ```
#[must_use]
#[instrument(level = Level::DEBUG, ret)]
pub fn configs_root() -> PathBuf {
    dirs::data_local_dir().join("configs").mkdir_if_not_exists()
}

/// Returns the path to a module-specific TOML configuration file.
///
/// # Arguments
///
/// * `module` - The name of the module, used as the filename (without extension)
///
/// # Examples
///
/// ```ignore
/// let db_config = config_path("database");
/// // Returns: {configs_root}/database.toml
/// ```
#[must_use]
#[instrument(level = Level::DEBUG, ret)]
pub fn config_path(module: &str) -> PathBuf {
    configs_root()
        .mkdir_if_not_exists()
        .join(format!("{module}.toml"))
}

/// Returns the root directory for application log files.
///
/// This creates a `logs` subdirectory within the application's local data directory.
/// The directory is created if it doesn't exist.
///
/// # Examples
///
/// ```ignore
/// let logs = logs_root();
/// // On Linux: ~/.local/share/{product_name}/logs
/// ```
#[must_use]
#[instrument(level = Level::DEBUG, ret)]
pub fn logs_root() -> PathBuf {
    dirs::data_local_dir().join("logs").mkdir_if_not_exists()
}

/// Returns a timestamped path for a new JSON log file.
///
/// The filename includes the current Unix epoch timestamp to ensure uniqueness
/// and chronological ordering.
///
/// # Format
///
/// The log file follows the pattern: `log_{epoch}.json`
///
/// # Examples
///
/// ```ignore
/// let log_file = log_path();
/// // Returns: {logs_root}/log_1707955200.json
/// ```
#[must_use]
#[instrument(level = Level::DEBUG, ret)]
pub fn log_path() -> PathBuf {
    logs_root()
        .mkdir_if_not_exists()
        .join(format!("log_{}.json", epoch()))
}
