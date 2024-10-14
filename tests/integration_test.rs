use minigrep::{Config, run};

#[test]
fn test_run() {
    let config = Config{
        query: String::from("ar"),
        file_path: String::from("./tests/test.txt"),
        ignore_case: false };
    assert!(run(config).is_ok())
}