// These tests can be ran locally but will significantly impact build time.
// Ideally `roche test` should be used It copies lib.rs, functions.rs to the dev container and runs `cargo test --lib` 
pub mod functions;

#[cfg(test)]
#[async_std::test]
async fn run_lib() {
    use tide_testing::TideTestingExt;
    let app = functions::handler();
    let resp_string = app.get("/").recv_string().await.unwrap();
    assert!(resp_string.contains("httpbin.org"));
}
