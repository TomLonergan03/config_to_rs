For converting config files to Rust structs, so they can be compiled in to a binary.
Allows for using config files in no-std no-alloc environments.

Work in progress.

Initially supports YAML.

## Feature flags
- "relative-to-macro": **requires nightly**, allows for paths relative to the macro
  invocation to be used in the `config_to_rs` attribute instead of paths relative to
  the crate root. This is useful if you have a complex project structure where you
  have multiple local crates in a single cargo project.

## Example

Given a YAML file:
```yaml
parsing: "working"
age: 22
enabled: true
time: 22.34
recurse:
  thing: 1
  recurse:
    thing: 3
array: ["a", "b", "c"]
empty_array: []
array_of_arrays:
  - ["a", "b", "c"]
  - ["d", "e", "f"]
array_of_objects:
  - name: "a"
    age: 1
  - name: "b"
    age: 2
  - name: "c"
    age: 3
```

and a Rust struct:
```rs
#[config_to_rs(yaml, path/to/config.yaml)]
pub struct Config;
```

the macro will generate the following Rust code:

```rs
struct Config {
    pub parsing: &'static str,
    pub age: i64,
    pub enabled: bool,
    pub time: f64,
    pub recurse: ConfigRecurse,
    pub array: [&'static str; 3usize],
    pub empty_array: [(); 0usize],
    pub array_of_arrays: [[&'static str; 3usize]; 2usize],
    pub array_of_objects: [ConfigArrayOfObjects; 3usize],
}

struct ConfigRecurse {
    pub thing: i64,
    pub recurse: ConfigRecurseRecurse,
}

struct ConfigRecurseRecurse {
    pub thing: i64,
}

struct ConfigArrayOfObjects {
    pub name: &'static str,
    pub age: i64,
}

pub const CONFIG: Config = Config {
    parsing: "working",
    age: 22i64,
    enabled: true,
    time: 22.34f64,
    recurse: ConfigRecurse {
        thing: 1i64,
        recurse: ConfigRecurseRecurse { thing: 3i64 },
    },
    array: ["a", "b", "c"],
    empty_array: [],
    array_of_arrays: [["a", "b", "c"], ["d", "e", "f"]],
    array_of_objects: [
        ConfigArrayOfObjects {
            name: "a",
            age: 1i64,
        },
        ConfigArrayOfObjects {
            name: "b",
            age: 2i64,
        },
        ConfigArrayOfObjects {
            name: "c",
            age: 3i64,
        },
    ],
};
```

Which then allows for accessing the config values as you would any other Rust struct:
```rs
// items in a hashmap
assert_eq!(CONFIG.parsing, "working");

// items in a list
assert_eq!(CONFIG.array[1], "b");

// nested hashmaps
assert_eq!(CONFIG.recurse.recurse.thing, 3);
```
