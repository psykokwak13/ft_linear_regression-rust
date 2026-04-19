use csv::ReaderBuilder;
use std::{
    env,
    io::Write,
    time::{Duration, Instant},
};

const TIME_TRAIN: u64 = 5;
const LEARNING_RATE: f64 = 0.0000000001;

fn estimate_price(mileage: i32, theta: (f64, f64)) -> f64 {
    theta.0 + theta.1 * (mileage as f64)
}

fn train(data: Vec<Vec<i32>>) -> (f64, f64) {
    let mut theta: (f64, f64) = (0.0, 0.0); // theta0, theta1 = 0.0
    let now = Instant::now();

    while now.elapsed().as_secs() != Duration::from_secs(TIME_TRAIN).as_secs() {
        theta.0 -= {
            // cost function
            let error = |x: &Vec<i32>| {
                estimate_price(*(x.get(0).expect("number not found")), theta)
                    - *(x.get(1).expect("number not found")) as f64
            };
            // gradient descent algorithm for theta0
            LEARNING_RATE * (1.0 / (data.len() as f64)) * data.iter().map(error).sum::<f64>()
        };
        theta.1 -= {
            // cost function
            let weighted_error = |x: &Vec<i32>| {
                (estimate_price(*(x.get(0).expect("number not found")), theta)
                    - *(x.get(1).expect("number not found")) as f64)
                    * *(x.get(0).expect("number not found")) as f64
            };
            // gradient descent algorithm for theta1
            LEARNING_RATE
                * (1.0 / (data.len() as f64))
                * data.iter().map(weighted_error).sum::<f64>()
        };
        // check for gradient explosion
        if theta.0.is_nan() || theta.1.is_nan() {
            println!("<NAN FOUND !!>");
            break;
        }
    }

    theta
}

// function used to put result of the training in a json file
fn put_json(theta: (f64, f64)) {
    let json_str = serde_json::to_string(&theta).expect("error in json"); // create and fill the string to put in the file
    let mut file = std::fs::File::create("data_theta.json").expect("error with file"); // create the file. if the file already exist we overwrite in
    file.write_all(json_str.as_bytes()) // write content of json_str in the file
        .expect("writing in json file failed.");
}

fn main() {
    // the program take one argument : the filename of the dataset
    if env::args().count() != 2 {
        panic!("./train <data filename>");
    }
    let mut rdr = ReaderBuilder::new() // parse and collect datas from the file
        .from_path(env::args().last().expect("error from file"))
        .unwrap();
    let data: Vec<Vec<i32>> = rdr // put datas in a Vec<Vec<i32>
        .records()
        .flatten()
        .map(|x| x.iter().map(|s| s.parse().unwrap()).collect())
        .collect();
    put_json(train(data));
}
