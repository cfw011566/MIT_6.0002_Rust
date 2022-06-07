mod drunk;
mod field;
mod location;

mod prelude {
    pub use crate::drunk::*;
    pub use crate::field::*;
    pub use crate::location::*;
}

use prelude::*;

fn main() {
    test_sanity();

    println!("durnk test");
    test_walk();
}

fn walk(f: &mut Field, d: &Drunk, num_steps: u32) -> f64 {
    let start = f.get_location(d);
    for _ in 0..num_steps {
        f.move_drunk(d);
    }
    let loc = f.get_location(d);
    start.distance_from(&loc)
}

fn test_sanity() {
    let p = Location::new(1.2, 2.3);
    println!("p = {}", p);

    let steps = vec![
        Location::new(0.0, 1.0),
        Location::new(0.0, -1.0),
        Location::new(1.0, 0.0),
        Location::new(-1.0, 0.0),
    ];
    let usual_drunk = Drunk::new("usual".to_owned(), &steps);
    println!("{}", usual_drunk);

    let steps = vec![
        Location::new(0.0, 1.1),
        Location::new(0.0, -0.9),
        Location::new(1.0, 0.0),
        Location::new(-1.0, 0.0),
    ];
    let masochist_drunk = Drunk::new("masochist".to_owned(), &steps);
    println!("{}", masochist_drunk);

    let mut f = Field::new();
    let origin = Location::new(0.0, 0.0);
    println!("Field {:?}", f);
    f.add_drunk(&usual_drunk, &origin);
    println!("add usual {:?}", f);
    f.add_drunk(&masochist_drunk, &origin);
    println!("add masochist {:?}", f);

    let dist = walk(&mut f, &usual_drunk, 10000);
    println!("distance = {}", dist);
    let dist = walk(&mut f, &masochist_drunk, 10000);
    println!("distance = {}", dist);
}

fn sim_walks(num_steps: u32, num_trials: u32, drunk: &Drunk) -> Vec<f64> {
    let origin = Location::new(0.0, 0.0);
    let mut distances: Vec<f64> = Vec::new();
    for _ in 0..num_trials {
        let mut f = Field::new();
        f.add_drunk(drunk, &origin);
        distances.push(walk(&mut f, drunk, num_steps));
    }
    distances
}

fn drunck_test(walk_lengths: &Vec<u32>, num_trials: u32, drunk: &Drunk) {
    for num_steps in walk_lengths {
        let distances = sim_walks(*num_steps, num_trials, drunk);
        println!("random walk of {} steps", num_steps);
        let sum: f64 = distances.iter().sum();
        let mut min = distances.get(0).unwrap().clone();
        let mut max = distances.get(0).unwrap().clone();
        for d in distances {
            if d > max {
                max = d;
            }
            if d < min {
                min = d;
            }
        }
        println!(" Mean = {}", sum / (num_trials as f64));
        println!(" Min = {}, Max = {}", min, max);
    }
}

fn test_walk() {
    let steps = vec![
        Location::new(0.0, 1.0),
        Location::new(0.0, -1.0),
        Location::new(1.0, 0.0),
        Location::new(-1.0, 0.0),
    ];
    let usual_drunk = Drunk::new("usual".to_owned(), &steps);

    let steps = vec![
        Location::new(0.0, 1.1),
        Location::new(0.0, -0.9),
        Location::new(1.0, 0.0),
        Location::new(-1.0, 0.0),
    ];
    let masochist_drunk = Drunk::new("masochist".to_owned(), &steps);

    let test_steps = vec![1000, 10000];
    println!("usual drunk test");
    drunck_test(&test_steps, 100, &usual_drunk);
    println!("masochist drunk test");
    drunck_test(&test_steps, 100, &masochist_drunk);
}
