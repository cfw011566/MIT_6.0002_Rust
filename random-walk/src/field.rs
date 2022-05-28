use super::drunk::Drunk;
use super::location::Location;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Field {
    drunks: HashMap<String, Location>,
}

impl Field {
    pub fn new() -> Self {
        let drunks: HashMap<String, Location> = HashMap::new();
        Self { drunks }
    }

    pub fn add_drunk(&mut self, drunk: &Drunk, location: &Location) {
        let name = drunk.name().to_string();
        self.drunks.insert(name, location.clone());
    }

    pub fn get_location(&self, drunk: &Drunk) -> Location {
        if let Some(loc) = self.drunks.get(drunk.name()) {
            loc.clone()
        } else {
            Location::new(0.0, 0.0)
        }
    }

    pub fn move_drunk(&mut self, drunk: &Drunk) {
        let name = drunk.name().to_string();
        if let Some(loc) = self.drunks.get(&name) {
            let mut loc = loc.clone();
            let (x_dist, y_dist) = drunk.take_step();
            loc.move_by(x_dist, y_dist);
            self.drunks.insert(name, loc);
        }
    }
}
