mod food;
use food::Food;

fn main() {
    let names = vec![
        "wine".to_string(),
        "beer".to_string(),
        "pizza".to_string(),
        "burger".to_string(),
        "fries".to_string(),
        "cola".to_string(),
        "apple".to_string(),
        "donut".to_string(),
        "cake".to_string(),
    ];
    let values = vec![89.0, 90.0, 95.0, 100.0, 90.0, 79.0, 50.0, 10.0];
    let calories = vec![123.0, 154.0, 258.0, 354.0, 365.0, 150.0, 95.0, 195.0];

    let foods = Food::build_menu(&names, &values, &calories);
    println!("The foods on menu");
    for f in foods.iter() {
        println!("  {}", f);
    }
    println!();

    test_greedys(&foods, 750.0);
    test_greedys(&foods, 800.0);
    test_greedys(&foods, 1000.0);
}

fn test_greedys(foods: &Vec<Food>, max_units: f64) {
    println!("Use greedy by value to allocate {} calories", max_units);
    let mut foods = foods.clone();
    let func = Box::new(|a: &Food, b: &Food| {
        let a_value = a.value();
        let b_value = b.value();
        b_value.partial_cmp(&a_value).unwrap()
    });
    foods.sort_by(func);
    let (taken, val) = greedy(&foods, max_units);
    println!("Total value of items taken = {}", val);
    for f in taken.iter() {
        println!("  {}", f);
    }
    println!();

    println!("Use greedy by cost to allocate {} calories", max_units);
    let func = Box::new(|a: &Food, b: &Food| {
        let a_calories = a.calories();
        let b_calories = b.calories();
        a_calories.partial_cmp(&b_calories).unwrap()
    });
    foods.sort_by(func);
    let (taken, val) = greedy(&foods, max_units);
    println!("Total value of items taken = {}", val);
    for f in taken.iter() {
        println!("  {}", f);
    }
    println!();

    println!("Use greedy by density to allocate {} calories", max_units);
    let func = Box::new(|a: &Food, b: &Food| {
        let a_density = a.density();
        let b_density = b.density();
        b_density.partial_cmp(&a_density).unwrap()
    });
    foods.sort_by(func);
    let (taken, val) = greedy(&foods, max_units);
    println!("Total value of items taken = {}", val);
    for f in taken.iter() {
        println!("  {}", f);
    }
    println!();
}

fn greedy(items: &Vec<Food>, max_cost: f64) -> (Vec<Food>, f64) {
    let mut results: Vec<Food> = Vec::new();
    let mut total_value: f64 = 0.0;
    let mut total_cost: f64 = 0.0;

    for item in items.iter() {
        let name = item.name().clone();
        let calories = item.calories().clone();
        let value = item.value().clone();
        if (total_cost + calories) <= max_cost {
            results.push(Food::new(name, value, calories));
            total_cost += calories;
            total_value += value;
        }
    }

    (results, total_value)
}
