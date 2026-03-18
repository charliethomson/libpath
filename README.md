# libpath

A Rust workspace providing platform-aware application directory utilities and product name management.

## Crates

### `libpath`

Wraps the [`dirs`](https://crates.io/crates/dirs) crate to return product-prefixed, auto-created paths for standard platform directories (cache, config, data, logs, etc.).

```rust
use libpath::dirs;

// Returns ~/.cache/{product_name}/, creating it if needed
let cache = dirs::cache_dir();

// Returns a path to a module-specific config file
let cfg = libpath::config_path("database"); // ~/.local/share/{product}/configs/database.toml

// Returns a timestamped log file path
let log = libpath::log_path(); // ~/.local/share/{product}/logs/log_{epoch}.json
```

All directory functions return a `PathBuf` with the product name appended, and create the directory if it does not exist.

#### `MkdirIfNotExists`

An extension trait on `PathBuf` for chaining directory creation:

```rust
use libpath::MkdirIfNotExists;
use std::path::PathBuf;

let p = PathBuf::from("/tmp/myapp/data").mkdir_if_not_exists();
```

### `libproduct`

Manages a global application `ProductName` with version, build metadata, and flexible descriptor formatting.

```rust
use libproduct::{ProductName, format};

let product = ProductName::new("dev.thmsn.myapp", "1.0.0", "myapp");
product.set_global().unwrap();

// "myapp v1.0.0"
println!("{}", ProductName::global().unwrap().descriptor(format::SHORT));

// "myapp v1.0.0 (main@abc1234)"
println!("{}", ProductName::global().unwrap().descriptor(format::DEFAULT));
```

The `product_name!` macro integrates build-time git and OS metadata automatically:

```rust
use libproduct::product_name;

const PRODUCT: libproduct::ProductName = product_name!("dev.thmsn.myapp", "1.0.0", "myapp");
```

#### Format strings

Pre-defined format strings in `libproduct::format`:

| Constant        | Example output                                      |
|-----------------|-----------------------------------------------------|
| `SHORT`         | `myapp v1.0.0`                                      |
| `DEFAULT`       | `myapp v1.0.0 (main@abc1234)`                       |
| `DEFAULT_DIRTY` | `myapp v1.0.0 (main@abc1234*)`                      |
| `LONG`          | includes build host and OS                          |
| `FULL`          | all available metadata                              |
| `USER_AGENT`    | suitable for HTTP `User-Agent` headers              |
| `GIT_REF_SHORT` | `main@abc1234`                                      |

Custom format strings may use any of the following placeholders:

`{NAME}`, `{VERSION}`, `{GIT_REF}`, `{GIT_HASH}`, `{GIT_LONG_HASH}`, `{GIT_DIRTY}`, `{GIT_DIRTY_STAR}`, `{GIT_MESSAGE}`, `{GIT_AUTHOR}`, `{GIT_EMAIL}`, `{GIT_TAGS}`, `{GIT_REMOTE}`, `{GIT_COMMIT_COUNT}`, `{BUILD_HOST}`, `{BUILD_OS}`, `{BUILD_OS_VERSION}`, `{BUILD_OS_LONG}`, `{BUILD_ARCH}`, `{BUILD_CPUS}`, `{BUILD_MEM}`

## Usage

Add the crates you need to your `Cargo.toml`:

```toml
[dependencies]
libpath = { git = "https://github.com/charliethomson/libpath.git" }
libproduct = { git = "https://github.com/charliethomson/libpath.git" }
```

`libpath` requires a global `ProductName` to be set before calling directory functions. Set it once at startup:

```rust
fn main() {
    libproduct::product_name!("com.example.myapp", "0.1.0", "myapp")
        .set_global()
        .unwrap();

    let log = libpath::log_path();
    // ...
}
```

## Status

Early alpha (`0.0.5-alpha.0`). APIs may change.
