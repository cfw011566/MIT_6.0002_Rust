mod drunk;
mod field;
mod location;

mod prelude {
    pub use crate::drunk::*;
    pub use crate::field::*;
    pub use crate::location::*;
}

use prelude::*;

use plotters::prelude::*;

fn main() {
    test_sanity();

    println!("drunk test");
    test_walk();

    println!("plot mean value of different drunk");
    test_plot_all();
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
    println!("New Field {:?}", f);
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

fn sim_drunk(num_trials: u32, drunk: &Drunk, walk_lengths: &Vec<u32>) -> Vec<f64> {
    let mut mean_distances: Vec<f64> = Vec::new();
    for num_steps in walk_lengths {
        println!("Start simulation of {num_steps} steps");
        let trials = sim_walks(*num_steps, num_trials, drunk);
        let sum: f64 = trials.iter().sum();
        let mean = sum / trials.len() as f64;
        mean_distances.push(mean);
    }
    mean_distances
}

fn sim_all(
    drunks: &Vec<Drunk>,
    walk_lengths: &Vec<u32>,
    num_trials: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("points.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE);
    let root = root.margin::<u32, u32, u32, u32>(20, 20, 20, 20);
    let title = format!("Mean Distance from Origin {num_trials} trials");

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 24).into_font())
        .x_label_area_size::<u32>(20)
        .y_label_area_size::<u32>(40)
        .build_cartesian_2d(0f32..100_000f32, 0f32..5_000f32)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.2}", x))
        .draw()?;

    for (i, drunk) in drunks.iter().enumerate() {
        println!("Start simulation of {}", drunk.name());
        let means = sim_drunk(num_trials, drunk, walk_lengths);
        println!("means = {:?}", means);
        let mut points: Vec<(f32, f32)> = Vec::new();
        for i in 0..walk_lengths.len() {
            let x = walk_lengths[i] as f32;
            let y = means[i] as f32;
            points.push((x, y));
        }
        for point in points.iter() {
            println!("{:?}", point);
        }
        let color = if i == 0 { RED } else { GREEN };
        chart.draw_series(LineSeries::new(points.clone(), &color))?;

        if i == 0 {
            chart.draw_series(points.iter().map(|point| Circle::new(*point, 5, &RED)))?;
        } else {
            chart.draw_series(
                points
                    .iter()
                    .map(|point| TriangleMarker::new(*point, 5, &GREEN)),
            )?;
        };
    }

    Ok(())
}

fn test_plot_all() {
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

    let drunks = vec![usual_drunk, masochist_drunk];
    let num_steps = vec![10, 100, 1000, 10_000, 100_000];
    sim_all(&drunks, &num_steps, 100);
}
