// WASM ? => react frontend to represent bezier curves
#[derive(Debug, Clone, Copy)]
struct Vector {
    x: f32,
    y: f32
}

fn main() {
    let vector1: Vector = Vector {x: 20.0, y:10.0};
    let vector2: Vector = Vector {x: 10.0, y:20.0};
    let point: Vector = Vector {x :15.0, y: 25.0 };
    // put the third vector in
    // vector1 -> P
    // p -> vector2
    // i => delta
    for i in 0..1000 {
        let v1: Vector = l_interpolation(vector1, point, (i as f32) / 1000.0);
        let v2: Vector = l_interpolation(point, vector2, (i as f32) / 1000.0);
        let cubic: Vector = l_interpolation(v1, v2, (i as f32) / 1000.0);
        println!("{:?}", cubic)
    }


}


fn l_interpolation(v1: Vector, v2: Vector, t: f32) -> Vector {
    let vec: Vector  =  Vector{
        x: lerp(v1.x, v2.x, t),
        y: lerp(v1.y, v2.y, t)
    };
    return vec;
}

fn lerp(x0: f32, x1: f32, t:f32) -> f32{
    return (1.0 - t) * x0 + t*x1 ;
}