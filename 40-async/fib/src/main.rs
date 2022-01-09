use futures::prelude::*;
use futures::stream::poll_fn;

#[tokio::main]
async sfn main() {
    println!("Hello, world!");
    consume(None);
}

async fn consume(mut st: impl Stream<Item = i32> + Unpin) {
    while let Some(v) = st.next().await {
        print!("{} ", v);
    }
    print!("\\n");
}
