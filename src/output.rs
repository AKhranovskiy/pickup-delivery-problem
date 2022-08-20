use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Move {
    time: u32,           // W
    train: String,       // T
    from: String,        // N1
    load: Vec<String>,   // P1
    to: String,          // N2
    unload: Vec<String>, // P2
}

impl Move {
    pub fn new(
        time: u32,
        train: String,
        from: String,
        load: Vec<String>,
        to: String,
        unload: Vec<String>,
    ) -> Self {
        Self {
            time,
            train,
            from,
            load,
            to,
            unload,
        }
    }
}

pub struct Output {
    moves: Vec<Move>,
    total_time: u32,
}

impl Output {
    pub fn new(moves: Vec<Move>, total_time: u32) -> Self {
        Self { moves, total_time }
    }

    pub fn total_time(&self) -> u32 {
        self.total_time
    }

    pub fn sort_by_time(&self) -> Self {
        Self {
            moves: self
                .moves
                .iter()
                .cloned()
                .sorted_by_key(|m| m.time)
                .collect(),
            total_time: self.total_time,
        }
    }

    pub fn sort_by_train(&self) -> Self {
        Self {
            moves: self
                .moves
                .iter()
                .cloned()
                .sorted_by_key(|m| m.train.clone())
                .collect(),
            total_time: self.total_time,
        }
    }
}

impl ToString for Output {
    fn to_string(&self) -> String {
        let mut result = self
            .moves
            .iter()
            .map(|m| {
                format!(
                    "W={}, T={}, N1={}, P1=[{}], N2={}, P2=[{}]",
                    m.time,
                    m.train,
                    m.from,
                    m.load.join(","),
                    m.to,
                    m.unload.join(","),
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        result.push('\n');
        result
    }
}
