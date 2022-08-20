use super::Station;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Order {
    name: String,
    weight: u32,
    location: Station,
    destination: Station,
}

impl Order {
    pub fn new(name: String, weight: u32, location: Station, destination: Station) -> Self {
        Self {
            name,
            weight,
            location,
            destination,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn weight(&self) -> u32 {
        self.weight
    }

    pub fn location(&self) -> Station {
        self.location.clone()
    }

    pub fn destination(&self) -> Station {
        self.destination.clone()
    }

    pub fn move_to(self, location: &Station) -> Self {
        Self {
            location: location.clone(),
            ..self
        }
    }

    pub fn is_delivered(&self) -> bool {
        self.location == self.destination
    }
}

impl From<(&str, u32, &str, &str)> for Order {
    fn from(tuple: (&str, u32, &str, &str)) -> Self {
        Self {
            name: tuple.0.to_string(),
            weight: tuple.1,
            location: tuple.2.into(),
            destination: tuple.3.into(),
        }
    }
}
