use std::sync::OnceLock;

pub mod __reexports {
    pub use libbuildinfo;
}

use libbuildinfo::BuildInfo;

static GLOBAL_PRODUCT_NAME: OnceLock<ProductName> = OnceLock::new();

/// Available format keys for use in [`ProductName::descriptor`] format strings:
///
/// | Key | Source | Example |
/// |-----|--------|---------|
/// | `{NAME}` | Product name (base + extensions) | `dev.thmsn.myapp` |
/// | `{VERSION}` | Cargo package version | `0.1.0` |
/// | `{GIT_REF}` | Branch name | `main` |
/// | `{GIT_HASH}` | Short commit hash | `7d712ab` |
/// | `{GIT_LONG_HASH}` | Full commit hash | `7d712abc048a...` |
/// | `{GIT_DIRTY}` | `dirty` if uncommitted changes, empty otherwise | `dirty` |
/// | `{GIT_MESSAGE}` | Commit message (trimmed) | `fix: login bug` |
/// | `{GIT_AUTHOR}` | Commit author name | `Charlie Thomson` |
/// | `{GIT_EMAIL}` | Commit author email | `charlie@thmsn.dev` |
/// | `{GIT_TAGS}` | Comma-separated tags | `v0.1.0,latest` |
/// | `{GIT_REMOTE}` | Remote URL | `git@github.com:...` |
/// | `{GIT_COMMIT_COUNT}` | Number of commits | `42` |
/// | `{BUILD_HOST}` | Build machine hostname | `Charless-MacBook-Pro.local` |
/// | `{BUILD_OS}` | OS name | `Darwin` |
/// | `{BUILD_OS_VERSION}` | OS version | `25.0.0` |
/// | `{BUILD_OS_LONG}` | OS long version | `macOS 26.0` |
/// | `{BUILD_ARCH}` | CPU architecture | `arm64` |
/// | `{BUILD_CPUS}` | Number of CPUs | `14` |
/// | `{BUILD_MEM}` | Total memory (human-readable) | `25.77 GB` |
///
/// Keys that resolve to `None` or are unavailable (e.g. git keys when
/// no `BuildInfo` is present) are replaced with an empty string.
pub mod format {
    /// `{NAME} v{VERSION}`
    pub const SHORT: &str = "{NAME} v{VERSION}";

    /// `{NAME} v{VERSION} ({GIT_REF}@{GIT_HASH})`
    pub const DEFAULT: &str = "{NAME} v{VERSION} ({GIT_REF}@{GIT_HASH})";

    /// `{NAME} v{VERSION} ({GIT_REF}@{GIT_HASH} {GIT_DIRTY_STAR})`
    pub const DEFAULT_DIRTY: &str = "{NAME} v{VERSION} ({GIT_REF}@{GIT_HASH}{GIT_DIRTY_STAR})";

    /// `{NAME} v{VERSION} | {GIT_REF}@{GIT_HASH} | {BUILD_HOST} ({BUILD_OS} {BUILD_OS_VERSION})`
    pub const LONG: &str =
        "{NAME} v{VERSION} | {GIT_REF}@{GIT_HASH} | {BUILD_HOST} ({BUILD_OS} {BUILD_OS_VERSION})";

    /// `{NAME} v{VERSION} | {GIT_REF}@{GIT_HASH} {GIT_DIRTY} | {GIT_AUTHOR} | {BUILD_HOST} ({BUILD_OS_LONG} {BUILD_ARCH})`
    pub const FULL: &str = "{NAME} v{VERSION} | {GIT_REF}@{GIT_HASH} {GIT_DIRTY} | {GIT_AUTHOR} | {BUILD_HOST} ({BUILD_OS_LONG} {BUILD_ARCH})";

    /// `{NAME}/{VERSION} ({BUILD_OS}; {BUILD_ARCH})`
    ///
    /// Useful as a User-Agent header value.
    pub const USER_AGENT: &str = "{NAME}/{VERSION} ({BUILD_OS}; {BUILD_ARCH})";

    /// `{GIT_REF}@{GIT_HASH}`
    pub const GIT_REF_SHORT: &str = "{GIT_REF}@{GIT_HASH}";
}

#[derive(Clone, Debug)]
pub struct ProductName {
    pub base: String,
    pub ext: Vec<String>,
    pub version: String,
    pub build: Option<BuildInfo>,
}
impl ProductName {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new<S1: ToString, S2: ToString>(
        base: S1,
        version: S2,
        package: Option<BuildInfo>,
    ) -> Self {
        Self {
            base: base.to_string(),
            ext: vec![],
            version: version.to_string(),
            build: package,
        }
    }

    #[must_use]
    // i dont care
    #[allow(clippy::needless_pass_by_value)]
    pub fn with<S: ToString>(&self, ext: S) -> Self {
        let mut this = self.clone();
        this.ext.push(ext.to_string());
        this
    }

    pub fn set_global(&self) -> Result<(), &'static str> {
        // Who the fuck cares
        GLOBAL_PRODUCT_NAME
            .set(self.clone())
            .map_err(|_| "Product name has already been set")
    }

    pub fn global() -> Option<ProductName> {
        // Who the fuck cares
        GLOBAL_PRODUCT_NAME.get().cloned()
    }

    #[must_use]
    pub fn name(&self) -> String {
        self.to_string()
    }

    #[must_use]
    pub fn version(&self) -> String {
        self.build
            .as_ref()
            .and_then(|build| build.package.version.clone())
            .unwrap_or(self.version.clone())
    }

    /// Format a descriptor string by replacing `{KEY}` placeholders with
    /// their resolved values. See [`format`] for available keys and
    /// pre-defined format constants.
    ///
    /// ```ignore
    /// let desc = product.descriptor("{NAME} v{VERSION} - {GIT_REF}@{GIT_HASH}");
    /// // => "dev.thmsn.myapp v0.1.0 - main@7d712ab"
    /// ```
    #[must_use]
    pub fn descriptor(&self, fmt: &str) -> String {
        let opt = |o: Option<&str>| o.unwrap_or("").to_string();

        let mut out = fmt
            .replace("{NAME}", &self.name())
            .replace("{VERSION}", &self.version());

        if let Some(build) = &self.build {
            let git = &build.git;
            let agent = &build.agent;

            out = out
                .replace("{GIT_REF}", &opt(git.branch.as_deref()))
                .replace("{GIT_LONG_HASH}", &git.commit_hash)
                .replace("{GIT_HASH}", &git.commit_short_hash)
                .replace("{GIT_DIRTY}", if git.dirty { "dirty" } else { "" })
                .replace("{GIT_DIRTY_STAR}", if git.dirty { "*" } else { "" })
                .replace("{GIT_MESSAGE}", opt(git.commit_message.as_deref()).trim())
                .replace("{GIT_AUTHOR}", &opt(git.author_name.as_deref()))
                .replace("{GIT_EMAIL}", &opt(git.author_email.as_deref()))
                .replace("{GIT_TAGS}", &git.tags.join(","))
                .replace("{GIT_REMOTE}", &opt(git.remote_url.as_deref()))
                .replace(
                    "{GIT_COMMIT_COUNT}",
                    &git.commit_count.map_or(String::new(), |c| c.to_string()),
                )
                .replace("{BUILD_HOST}", &agent.hostname)
                .replace("{BUILD_OS}", &agent.os.name)
                .replace("{BUILD_OS_VERSION}", &agent.os.version)
                .replace("{BUILD_OS_LONG}", &agent.os.long_version)
                .replace("{BUILD_ARCH}", &agent.os.architecture)
                .replace("{BUILD_CPUS}", &agent.ncpus.to_string())
                .replace("{BUILD_MEM}", &agent.memory.total.human);
        } else {
            out = out
                .replace("{GIT_REF}", "")
                .replace("{GIT_LONG_HASH}", "")
                .replace("{GIT_HASH}", "")
                .replace("{GIT_DIRTY}", "")
                .replace("{GIT_MESSAGE}", "")
                .replace("{GIT_AUTHOR}", "")
                .replace("{GIT_EMAIL}", "")
                .replace("{GIT_TAGS}", "")
                .replace("{GIT_REMOTE}", "")
                .replace("{GIT_COMMIT_COUNT}", "")
                .replace("{BUILD_HOST}", "")
                .replace("{BUILD_OS}", "")
                .replace("{BUILD_OS_VERSION}", "")
                .replace("{BUILD_OS_LONG}", "")
                .replace("{BUILD_ARCH}", "")
                .replace("{BUILD_CPUS}", "")
                .replace("{BUILD_MEM}", "");
        }

        out
    }
}
impl std::fmt::Display for ProductName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)?;
        for ext in &self.ext {
            write!(f, ".{ext}")?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! product_name {
    (as $as:ident from $from:expr) => {
        pub const $as: std::sync::LazyLock<$crate::ProductName> = std::sync::LazyLock::new(|| $from );
    };

    ($base:literal) => {
        product_name!(with base $base as PRODUCT_NAME);
    };

    (with base $base:literal as $as:ident) => {
        product_name!(as $as from $crate::ProductName::new($base, std::env!("CARGO_PKG_VERSION"), $crate::__reexports::libbuildinfo::load_build_info!(optional).transpose().expect("Failed to load build info")));
    };

    (from $from:ident with extension $ext:literal) => {
        product_name!(as PRODUCT_NAME from $from.with($ext));
    };

    (from $from:ident with extension $ext:literal as $as:ident) => {
        product_name!(as $as from $from.with($ext));
    };
}

#[cfg(test)]
mod macro_tests {

    mod standard {
        product_name!("dev.thmsn.standard");

        product_name!(with base "dev.thmsn.as" as TEST_PRODUCT_NAME);
    }
    mod ext {
        product_name!(with base "dev.thmsn.ext" as BASE_PRODUCT_NAME);

        product_name!(from BASE_PRODUCT_NAME with extension "app");
        product_name!(from BASE_PRODUCT_NAME with extension "app_with_as" as PRODUCT_NAME_EXT_APP);
        product_name!(from PRODUCT_NAME with extension "another_ext" as PRODUCT_NAME_EXT_TWICE);
    }

    #[test]
    fn test_base_case() {
        assert_eq!(standard::PRODUCT_NAME.to_string(), "dev.thmsn.standard");
    }

    #[test]
    fn test_base_with_as_case() {
        assert_eq!(standard::TEST_PRODUCT_NAME.to_string(), "dev.thmsn.as");
    }

    #[test]
    fn test_ext_case() {
        assert_eq!(ext::PRODUCT_NAME.to_string(), "dev.thmsn.ext.app");
    }

    #[test]
    fn test_ext_with_as_case() {
        assert_eq!(
            ext::PRODUCT_NAME_EXT_APP.to_string(),
            "dev.thmsn.ext.app_with_as"
        );
    }

    #[test]
    fn test_ext_with_ext_on_ext_case() {
        assert_eq!(
            ext::PRODUCT_NAME_EXT_TWICE.to_string(),
            "dev.thmsn.ext.app.another_ext"
        );
    }
}
