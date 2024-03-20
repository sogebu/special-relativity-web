use shape::{BuildData, VertexPositionNormal};

fn main() {
    let cube = shape::IcosahedronOption::new()
        .radius(5.0)
        .build::<VertexPositionNormal>();
    let mut buf = Vec::new();
    cube.write_as_obj(&mut buf).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}
