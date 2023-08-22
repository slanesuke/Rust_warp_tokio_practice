use warp::Filter;
use std::collections::HashMap;

// First time using warp and tokio and making path, get, and query calls

async fn hello(param: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("{:#?}", param))
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    // let hello = warp::path!("hello" / String)
    //     .map(|name| format!("Hello, {}!", name));

    let hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::query::<HashMap<String, String>>()) // "127.0.0.1:3030/hello?name=NAME"
        .and(warp::path::end())
        .and_then(hello);

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}