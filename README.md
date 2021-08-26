# owmath
Quaternions

# Example:
```rust
use owmath::quaternion::Quaternion;

fn main() {
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
```
# Output:
```
(1, 2, 3, 4) + (2, 3, 4, 5) = (3, 5, 7, 9)
(1, 2, 3, 4) * (2, 3, 4, 5) = (-36, 6, 12, 12)
(2, 3, 4, 5) * (1, 2, 3, 4) = (-36, 8, 8, 14)
2 * (1, 2, 3, 4) = (2, 4, 6, 8)
(1, 2, 3, 4) * 2 = (2, 4, 6, 8)
(1, 2, 3, 4)⁻¹ = (0.03333333333333333, -0.06666666666666667, -0.1, -0.13333333333333333)
(1, 2, 3, 4).norm() = 5.477225575051661
(1, 2, 3, 4).conjugate() = (1, -2, -3, -4)
```
