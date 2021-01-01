use tide::prelude::*; // Pulls in the json! macro.

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
    api.at("/animals").get(|_| async {
        Ok(json!({
            "meta": { "count": 2 },
            "animals": [
                { "type": "cat", "name": "chashu" },
                { "type": "cat", "name": "nori" }
            ]
        }))
    });
    api
}