struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

enum Relation {
    Identical,
    Parallel,
    Intersecting(Vec3),
    Skew,
}


trait Lalgebra {
    fn dot(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            x: (v1.x * v2.x),
            y: (v1.y * v2.y),
            z: (v1.z * v2.z),
        }
    }
    fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            x: (v1.y * v2.z) - (v1.z * v2.y),
            y: (v1.z * v2.x) - (v1.x * v2.z),
            z: (v1.x * v2.y) - (v1.y * v2.x),
        }
    }
}

impl Lalgebra for Vec3 {}

fn main() {
    println!("Hello, world!");
}

asdf



