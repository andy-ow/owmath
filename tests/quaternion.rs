use owmath::quaternion::Quaternion;

#[test]
fn main_test() {
    let a = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let b: Quaternion<f64> = (2.0, 3.0, 4.0, 5.0).into();
    println!("{} + {} = {}", a, b, a+b);
    println!("{} * {} = {}", a, b, a*b);
    println!("{} * {} = {}", b, a, b*a);
    println!("2 * {} = {}", a, 2.0*a);  // works only for Quaternion<f32> and Quaternion<f64>
    println!("{} * 2 = {}", a, a*2.0);
    println!("{}⁻¹ = {}", a, a.inverse());
    println!("{}.norm() = {}", a, a.norm());
    println!("{}.conjugate() = {}", a, a.conjugate());
}