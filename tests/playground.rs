use std::path::Path;

#[test]
#[ignore]
fn playground() {
    let path = Path::new("C:\\Users\\rexies").join("Hello World");
    println!("{}", path.display());
}
