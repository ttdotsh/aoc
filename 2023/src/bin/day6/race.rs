pub struct Race {
    time_allowed: u64,
    distance_record: u64,
}

impl From<(u64, u64)> for Race {
    fn from(value: (u64, u64)) -> Self {
        Self {
            time_allowed: value.0,
            distance_record: value.1,
        }
    }
}

impl Race {
    pub fn num_ways_to_win(&self) -> u64 {
        let hold_times = 1..self.time_allowed;

        hold_times
            .filter_map(|ht| {
                let travel_time = self.time_allowed - ht;
                let distance = ht * travel_time;

                (distance > self.distance_record).then_some(distance)
            })
            .count() as u64
    }
}
