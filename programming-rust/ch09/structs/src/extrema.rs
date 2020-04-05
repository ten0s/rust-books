pub struct Extrema<'s> {
    min: &'s i32,
    max: &'s i32,
}

impl<'s> Extrema<'s> {
    pub fn new(slice: &'s [i32]) -> Self {
        let mut min = &slice[0];
        let mut max = &slice[0];

        for i in 1..slice.len() {
            if slice[i] < *min {
                min = &slice[i];
            }
            if slice[i] > *max {
                max = &slice[i];
            }
        }
        Extrema { min, max }
    }

    pub fn min(&self) -> i32 {
        *self.min
    }

    pub fn max(&self) -> i32 {
        *self.max
    }
}
