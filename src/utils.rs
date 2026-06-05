pub fn print_listener_info(listener: &tokio::net::TcpListener) {
    let routes = vec![
        "🟢 GET     /",
        "",
        "🟢 GET     /quote/{id}",
        "🟡 PUT     /quote/{id}",
        "",
        "🟢 GET     /quote/daily",
        "🟢 GET     /quote/random",
    ];

    let listener_addr = listener
        .local_addr()
        .expect("Failed to get listener address");

    println!("listening on http://{}", listener_addr);

    for route in routes {
        println!("{route}");
    }
}
