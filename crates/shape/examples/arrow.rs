use shape::VertexPositionNormal;

fn main() {
    let cube = shape::ArrowOption::new()
        .div(6)
        .build::<VertexPositionNormal>();
    let mut buf = Vec::new();
    cube.write_as_obj(&mut buf).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}
