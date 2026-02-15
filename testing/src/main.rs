use libproduct::{ProductName, product_name};

product_name!("dev.thmsn.libpath.testing");

fn main() {
    PRODUCT_NAME.set_global().unwrap();

    let name = ProductName::global().unwrap();
    println!(
        "SHORT:         {}",
        name.descriptor(libproduct::format::SHORT)
    );
    println!(
        "DEFAULT:       {}",
        name.descriptor(libproduct::format::DEFAULT)
    );
    println!(
        "DEFAULT_DIRTY: {}",
        name.descriptor(libproduct::format::DEFAULT_DIRTY)
    );
    println!(
        "LONG:          {}",
        name.descriptor(libproduct::format::LONG)
    );
    println!(
        "FULL:          {}",
        name.descriptor(libproduct::format::FULL)
    );
    println!(
        "USER_AGENT:    {}",
        name.descriptor(libproduct::format::USER_AGENT)
    );
    println!(
        "GIT_REF_SHORT: {}",
        name.descriptor(libproduct::format::GIT_REF_SHORT)
    );
    println!(
        "CUSTOM:        {}",
        name.descriptor("{NAME} v{VERSION}, {GIT_REF}")
    );
}
