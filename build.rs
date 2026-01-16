fn main() {
    let mut b = freertos_build::Builder::new();
    b.user_config_dir("src_c");
    b.compile().unwrap();
}
