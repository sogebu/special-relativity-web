use shape::{BuildData, VertexPositionNormal};

fn main() {
    let cube = shape::ArrowOption::new()
        .division_n(6)
        .build::<VertexPositionNormal>();
    let mut buf = Vec::new();
    cube.write_as_obj(&mut buf).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}
