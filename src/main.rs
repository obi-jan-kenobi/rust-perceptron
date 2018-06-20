extern crate rand;

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

fn main() {
    let mut points = Vec::new();
    let mut weights = vec![1.0,-1.0,1.0];
    let learningrate = 0.003;
    for _ in 0..99 {
        points.push(vec![1.0,2.0,3.0]);
    };
    points.iter().for_each(|v| weights = train(learningrate, v, &weights, 1.0));
    println!("{}", activation(weighted_sum(&vec![1.0,2.0,3.0], &weights)));
}
