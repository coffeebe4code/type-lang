use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    kind: String,
    name: String,
    details: serde_yaml::Value,
}

pub struct ProjectConfig {}

impl ProjectConfig {
    pub fn new() -> ProjectConfig {
        ProjectConfig {}
    }
    pub fn parse_multi_yaml(&mut self, file: String) -> () {
        for document in serde_yaml::Deserializer::from_str(&file) {
            let v = Recipe::deserialize(document).unwrap();
            println!("{:?}", v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let yaml = "\
        kind: project\n\
        name: SDL\n\
        details:\n\
            - ref: help\n\
            - me\n\
        ---\n\
        kind: target\n\
        name: sdl_exe\n\
        details:\n\
            - help\n\
            - me\n\
        "
        .to_string();
        let mut project = ProjectConfig::new();
        project.parse_multi_yaml(yaml);
    }
}
