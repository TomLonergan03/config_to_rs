use saphyr::Yaml;

pub fn is_terminal_type(value: &Yaml) -> bool {
    match value {
        Yaml::String(_) => true,
        Yaml::Real(_) => true,
        Yaml::Integer(_) => true,
        Yaml::Boolean(_) => true,
        _ => false,
    }
}
