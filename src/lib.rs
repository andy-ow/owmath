#![feature(trait_alias)]

mod quaternion;

pub use self::quaternion::Quaternion;

#[cfg(not(tarpaulin_include))]
#[allow(dead_code)]
fn lib() {
    let a = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let b = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let _c = a + b;
}
