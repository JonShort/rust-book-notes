// general.rs

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, n: i32) {
        self.list.push(n);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(x) => {
                self.update_average();
                Some(x)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    // this is private
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut list = AveragedCollection {
        list: vec![],
        average: 0.0,
    };
    println!("init = {}", list.average());

    list.add(1);
    list.add(3);
    println!("add 1, 3 = {}", list.average());

    list.add(2);
    list.add(3);
    println!("add 2, 3 = {}", list.average());

    let rem = list.remove();
    println!("removed {:?} = {}", rem, list.average());
}
