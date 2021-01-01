pub mod functions;

#[cfg(test)]
#[async_std::test]
async fn run_lib() {
    use tide_testing::TideTestingExt;
    let app = functions::handler();
    let resp_string = app.get("/").recv_string().await.unwrap();
    assert!(resp_string.contains("httpbin.org"));
}

#[async_std::test]
async fn json_test() {
    use tide::prelude::*;
    use tide_testing::TideTestingExt;
    let app = functions::handler();
    let response_json : serde_json::value::Value = app.get("/animals").recv_json().await.unwrap();

    assert_eq!(
        response_json,
        json!({
            "meta": { "count": 2 },
            "animals": [
                { "type": "cat", "name": "chashu" },
                { "type": "cat", "name": "nori" }
            ]
        })
    );
}


