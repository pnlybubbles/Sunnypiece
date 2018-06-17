mod math;

use math::*;

fn main() {
  let a = Vector3::new(1.0, 2.0, 3.0);
  let b = Vector3::new(4.0, 5.0, 6.0);
  println!("a = {}", a);
  println!("b = {}", b);
  println!("a + b = {}", a + b);
  println!("a - b = {}", a - b);
  println!("a * 2 = {}", a * 2.0);
  println!("2 * a = {}", 2.0 * a);
  println!("a / 2 = {}", a / 2.0);
  println!("2 / a = {}", 2.0 / a);
  println!("a . b = {}", a.dot(b));
  println!("a x b = {}", a.cross(b));
  println!("|a| = {}", a.norm());
  println!("a / |a| = {}", a.normalize());
  println!("2 ~= 2. = {}", 2.0.approx_eq(2.0 + EPS));
}
