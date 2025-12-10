use libproduct::{ProductName, product_name};

product_name!("dev.thmsn.libpath.testing");

fn main() {
    PRODUCT_NAME.set_global().unwrap();

    let name = ProductName::global().unwrap();
    println!("Global name: {name}");
}
