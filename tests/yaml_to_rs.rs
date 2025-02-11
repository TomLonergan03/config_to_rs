use config_to_rs::config_to_rs;

#[config_to_rs("tests/test.yaml")]
pub struct Config;

#[test]
fn test_config() {
    assert_eq!(CONFIG.time, 22.34f64);
}
