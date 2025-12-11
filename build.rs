fn main() {
    let mut b = freertos_build::Builder::new();
    b.freertos("FreeRTOS-Kernel/");
    b.freertos_config("src_c");
    b.compile().unwrap_or_else(|e| panic!("{}", e));
}
