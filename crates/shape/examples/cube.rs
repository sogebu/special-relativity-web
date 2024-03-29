use shape::{BuildData, VertexPositionCalcNormal, VertexPositionNormal};

fn main() {
    let cube = shape::CubeOption::new()
        .size(2.0)
        .center([0.0, 0.0, 1.0])
        .build::<VertexPositionCalcNormal>()
        .vertex_converted::<VertexPositionNormal>();
    let mut buf = Vec::new();
    cube.write_as_obj(&mut buf).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}
