mod models;
mod routes;
mod services;

use routes::{
    quote::get_daily_quote,
    quote::get_quote_by_id,
    quote::get_random_quote,
    root::root,
    // quote::create_quote, quote::delete_quote, quote::update_quote,
};

use axum::{
    Router,
    routing::get,
    // routing::{delete, post, put},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/quote/{id}", get(get_quote_by_id))
        .route("/quote/daily", get(get_daily_quote))
        .route("/quote/random", get(get_random_quote));
    //.route("/quote", post(create_quote))
    //.route("/quote/{id}", put(update_quote))
    //.route("/quote/{id}", delete(delete_quote))

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    print_listener_info(&listener);
    axum::serve(listener, app).await.unwrap();
}

fn print_listener_info(listener: &tokio::net::TcpListener) {
    let routes = vec![
        "🟢 GET     /",
        "",
        "🟢 GET     /quote/{id}",
        "",
        "🟢 GET     /quote/daily",
        "🟢 GET     /quote/random",
        //"🔵 POST    /quote/{id}",
        //"🟡 PUT     /quote/{id}",
        //"🔴 DELETE  /quote/{id}",
    ];
    println!("listening on http://{}", listener.local_addr().unwrap());
    for route in routes {
        println!("{route}");
    }
}
