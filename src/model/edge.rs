use super::Station;

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    name: String,
    stations: (Station, Station),
    distance: u32,
}

impl Edge {
    pub fn new(name: String, from: Station, to: Station, distance: u32) -> Self {
        Self {
            name,
            stations: (from, to),
            distance,
        }
    }

    pub fn stations(&self) -> &(Station, Station) {
        &self.stations
    }

    pub fn distance(&self) -> u32 {
        self.distance
    }
}

impl From<(&str, &str, &str, u32)> for Edge {
    fn from(data: (&str, &str, &str, u32)) -> Self {
        Self {
            name: data.0.to_string(),
            stations: (data.1.into(), data.2.into()),
            distance: data.3,
        }
    }
}
