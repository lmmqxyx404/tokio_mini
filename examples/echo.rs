use std::error::Error;

fn main() {
    // 创建runtime
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    rt.block_on(async {
        println!("hello  world");
    })
}
