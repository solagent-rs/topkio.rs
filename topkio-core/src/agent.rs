#[derive(Debug, Clone)]
pub enum Instructions {
    String(String),
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub name: String,
    pub instructions: Option<Instructions>,
}

impl Agent {
    pub fn new(name: &str, instructions: Option<Instructions>) -> Self {
        Agent {
            name: name.to_string(),
            instructions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_works() {
        let agent = Agent::new("Assistant", None);
        let new_agent = agent.clone();

        assert_eq!(agent.name, new_agent.name);
    }
}
