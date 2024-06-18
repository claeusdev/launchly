use std::fmt::Display;

fn main() {
    let feature_toggles = Lever::from_json("some_json_file_name".to_string());
    for t in &feature_toggles {
        println!("{t}")
    }
}

struct Lever {
    toggles: Vec<Toggle>,
}

impl Lever {
    fn toggles(&self) -> &[Toggle] {
        &self.toggles
    }

    fn from_json(file_name: String) -> Vec<Toggle> {
        // load json file
        // parse each object as a Toggle
        // load all toggles to memory
        let toggle = Toggle::new(
            "pfx_123".to_string(),
            "A very simple toggle".to_string(),
            ToggleStatus::Off,
        );
        println!("{file_name}");
        println!(
            "name: {}, description: {}, status: {}",
            toggle.name, toggle.description, toggle.status
        );
        return Vec::new();
    }
}

struct Toggle {
    name: String,
    description: String,
    status: ToggleStatus,
}

impl Display for Toggle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, description: {}, status: {}",
            self.name, self.description, self.status
        )
    }
}

enum ToggleStatus {
    On,
    Off,
}

impl ToggleStatus {
    fn status(&self) -> &str {
        match self {
            Self::On => "On",
            Self::Off => "Off",
        }
    }
}

impl Display for ToggleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.status())
    }
}

impl Toggle {
    pub fn new(name: String, description: String, status: ToggleStatus) -> Self {
        return Self {
            name,
            description,
            status,
        };
    }
}
