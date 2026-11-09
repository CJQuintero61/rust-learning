// defines the AveragedCollection struct and its methods
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    // constructor
    pub fn new() -> Self {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0
        }
    }


    // public method to add to the list
    pub fn add(&mut self, value: i32) {
        // private by default unless 'pub' is written
        self.list.push(value);
        self.update_average();
    }

    // public method to remove from the list
    pub fn remove(&mut self) -> Option<i32> {
        // since this method returns an Option<i32> 
        // the result will either be None or Some(value)
        let result = self.list.pop();
        match result {
            Some(value) => { // unwrap the i32 inside the Some variant, meaning the pop returned an Option<i32> that isn't None
                self.update_average();
                Some(value) // re-wrap the value that was popped back into an Option<i32> since the method returns an Option<i32>
            }
            // pop returned an Option<i32> with the None variant
            None => None,
        }
    }
    // public method to get the average
    pub fn average(&self) -> f64 {
        self.average
    }

    // private method to update the average
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}