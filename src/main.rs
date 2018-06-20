extern crate rand;

use rand::{thread_rng, Rng};

fn weighted_sum(inputs: &Vec<f64>, weights: &Vec<f64>) -> f64 {
    inputs.iter().zip(weights.iter()).fold(0.0, |state, (i, w)| { state + i * w })
}

fn activation(weighted_sum: f64) -> f64 {
    if weighted_sum < 0.0 { -1.0 } else { 1.0 }
}

fn train(learningrate: f64, inputs: &Vec<f64>, weights: &Vec<f64>, desired: f64) -> Vec<f64> {
    let guess = activation(weighted_sum(&inputs, &weights));
    let error = desired - guess; 
    weights.iter().zip(inputs.iter()).map(|(_w, i)| learningrate * error * i).collect()
}

fn f(x: f64) -> f64 {
    0.3 * x + 0.4
}

fn main() {
    let mut rng = thread_rng();
    let mut points = Vec::new();
    let mut weights = vec![rng.gen_range(-1.0,1.0), rng.gen_range(-1.0,1.0)];
    let mut answers = Vec::new();
    let learningrate = 0.003;
    for _ in 0..10000 {
        let x = rng.gen_range(-1.0,1.0);
        let y = rng.gen_range(-1.0,1.0);
        points.push(vec![x, y]);
        answers.push(if y < f(x) { -1.0 } else { 1.0 });
    };
    points.iter().zip(answers.iter()).for_each(|(p, a)| weights = train(learningrate, p, &weights, *a));
    println!("{}", activation(weighted_sum(&vec![0.3,0.7], &weights)));
}
