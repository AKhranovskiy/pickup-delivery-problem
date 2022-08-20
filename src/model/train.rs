use super::Station;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Train {
    name: String,
    capacity: u32,
    location: Station,
    traveled_time: u32,
}

impl Train {
    pub fn new(name: String, capacity: u32, location: Station) -> Self {
        Self {
            name,
            capacity,
            location,
            traveled_time: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn location(&self) -> &Station {
        &self.location
    }

    pub fn move_to(self, destination: &Station, traveled_time: u32) -> Self {
        Self {
            location: destination.clone(),
            traveled_time: self.traveled_time + traveled_time,
            ..self
        }
    }

    pub fn traveled_time(&self) -> u32 {
        self.traveled_time
    }
}

impl From<(&str, u32, &str)> for Train {
    fn from(tuple: (&str, u32, &str)) -> Self {
        Self {
            name: tuple.0.to_string(),
            capacity: tuple.1,
            location: tuple.2.into(),
            traveled_time: 0,
        }
    }
}
