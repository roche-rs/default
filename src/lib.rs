// Local unit tests are currently possible using the format below but will significantly impact build time.
// Ideally `roche test` should copy lib.rs, tests/ and functions.rs to the dev container and run them 
pub mod functions;

#[cfg(test)]
#[async_std::test]
async fn run_lib() {
    use tide_testing::TideTestingExt;
    let app = functions::handler();
    let resp_string = app.get("/").recv_string().await.unwrap();
    assert!(resp_string.contains("httpbin.org"));
}
