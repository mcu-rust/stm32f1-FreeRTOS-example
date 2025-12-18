fn main() {
    let mut b = freertos_build::Builder::new();
    b.freertos_config("src_c");
    b.compile().unwrap();
}
