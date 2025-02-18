use config_to_rs::config_to_rs;

#[cfg(feature = "relative-to-macro")]
#[config_to_rs(yaml, ../test.yaml)]
pub struct Config;

#[cfg(not(feature = "relative-to-macro"))]
#[config_to_rs(yaml, test.yaml)]
pub struct Config;

#[test]
fn test_config() {
    assert_eq!(CONFIG.parsing, "working");
    assert_eq!(CONFIG.age, 22i64);
    assert_eq!(CONFIG.enabled, true);
    assert_eq!(CONFIG.time, 22.34f64);
    assert_eq!(CONFIG.recurse.thing, 1i64);
    assert_eq!(CONFIG.recurse.recurse.thing, 3i64);
    assert_eq!(CONFIG.array, ["a", "b", "c"]);
    assert_eq!(CONFIG.empty_array, []);
    assert_eq!(CONFIG.array_of_arrays, [["a", "b", "c"], ["d", "e", "f"]]);
    for (i, obj) in CONFIG.array_of_objects.iter().enumerate() {
        assert_eq!(obj.name, ['a', 'b', 'c'][i].to_string());
        assert_eq!(obj.age, (i + 1) as i64);
    }
}
