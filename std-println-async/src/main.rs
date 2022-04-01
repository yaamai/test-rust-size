use async_executor::Executor;
use futures_lite::future;

fn main() {
    let ex = Executor::new();
    let task = ex.spawn(async {
        println!("Hello world");
    });
    future::block_on(ex.run(task));
}
