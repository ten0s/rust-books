pub struct AveragedCollection {
    list: Vec<i32>,
    avg: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            list: vec![],
            avg: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_avg();
    }

    pub fn remove(&mut self) -> Option<i32> {
        match self.list.pop() {
            Some(value) => {
                self.update_avg();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.avg
    }

    fn update_avg(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.avg = total as f64 / self.list.len() as f64;
    }
}
