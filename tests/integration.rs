#[cfg(test)]
#[async_std::test]
// This test will fail unless you are running the service
// See README.md for the steps
async fn it_works() {
    let uri = "http://localhost:8080/";
    let resp_string: String = match surf::get(uri).recv_string().await {
        Ok(result) => result,
        Err(_) => "".to_string(),
    };

    assert!(resp_string.contains("httpbin.org"));
}
