use crate::alg::Vector3;

mod alg;
mod rigidbody;

fn main() {
    println!("Hello, world!");

    let a = Vector3 { x: 3., y: 5., z: 2. };
    let b = Vector3 { x: 6., y: 1., z: 7. };

    println!("{a}");
    println!("{b}");
    println!("{}", a.cross(b));
}
