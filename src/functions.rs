pub fn handler() -> tide::Server<()> {
    let mut api = tide::new();
    api.at("/").get(|req: tide::Request<()>| async move {
        // See request attributes
        // https://docs.rs/tide/0.15.0/tide/struct.Request.html
        println!("Request Method : {}", req.method());

        // See surf documentation
        // https://docs.rs/surf/2.1.0/surf/
        let uri = "https://httpbin.org/get";
        let string: String = surf::get(uri).recv_string().await?;
        Ok(string)
    });
    api
}
