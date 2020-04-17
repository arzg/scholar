use scholar::dataset::Dataset;
use scholar::net::{NeuralNet, Sigmoid};

fn main() {
    let data = vec![
        (vec![0.0, 0.0], vec![0.0]),
        (vec![0.0, 1.0], vec![1.0]),
        (vec![1.0, 0.0], vec![1.0]),
        (vec![1.0, 1.0], vec![0.0]),
    ];

    let dataset = Dataset::from(data);

    let mut brain = NeuralNet::new(&[2, 10, 10, 1], Sigmoid);
    brain.train(dataset, 250_000, 0.01);

    println!("Prediction: {:.2}", brain.guess(&[1.0, 1.0])[0]);
}