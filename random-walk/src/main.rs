mod location;
use location::Location;

mod drunk;
use drunk::Drunk;

mod field;
use field::Field;

fn main() {
    test_sanity();
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
    let usual_drunk = Drunk::new(String::from("usual"), &steps);
    println!("{}", usual_drunk);

    let steps = vec![
        Location::new(0.0, 1.1),
        Location::new(0.0, -0.9),
        Location::new(1.0, 0.0),
        Location::new(-1.0, 0.0),
    ];
    let masochist_drunk = Drunk::new(String::from("masochist"), &steps);
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
