use std::sync::OnceLock;

static GLOBAL_PRODUCT_NAME: OnceLock<ProductName> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct ProductName {
    pub base: String,
    pub ext: Vec<String>,
}
impl ProductName {
    pub fn new<S: ToString>(base: S) -> Self {
        Self {
            base: base.to_string(),
            ext: vec![],
        }
    }

    pub fn with<S: ToString>(&self, ext: S) -> Self {
        let mut this = self.clone();
        this.ext.push(ext.to_string());
        this
    }

    pub fn set_global(&self) -> Result<(), ProductName> {
        // Who the fuck cares
        GLOBAL_PRODUCT_NAME.set(self.clone())
    }

    pub fn global() -> Option<ProductName> {
        // Who the fuck cares
        GLOBAL_PRODUCT_NAME.get().cloned()
    }
}
impl std::fmt::Display for ProductName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)?;
        for ext in self.ext.iter() {
            write!(f, ".{}", ext)?;
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
        product_name!(as $as from $crate::ProductName::new($base));
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
