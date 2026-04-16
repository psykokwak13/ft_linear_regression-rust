use csv::ReaderBuilder;
use std::{env, time::{Duration, Instant}};

const TIME_TRAIN : u64 = 60;
const LEARNING_RATE : f64 = 0.0000000001;

fn estimate_price(mileage : i32, theta : (f64, f64)) -> f64{
	theta.0 + theta.1 * (mileage as f64)
}

fn summation0(data : &Vec<Vec<i32>>, theta : (f64, f64)) -> f64 {
	let mut result = 0.0f64;

	for d in data.iter() {
		result += estimate_price(*(d.get(0).expect("number not found")), theta) - *(d.get(1).expect("number not found")) as f64;
	}
	result
}

fn summation1(data : &Vec<Vec<i32>>, theta : (f64, f64)) -> f64 {
	let mut result = 0.0f64;

	for d in data.iter() {
		result += (estimate_price(*(d.get(0).expect("number not found")), theta) - *(d.get(1).expect("number not found")) as f64) * *(d.get(0).expect("number not found")) as f64;
	}
	result
}

fn train(data : Vec<Vec<i32>>) -> (f64, f64) {
	let mut theta : (f64, f64) = (0.0, 0.0); // theta0, theta1 = 0.0

	let now = Instant::now();
	while now.elapsed().as_secs() != Duration::from_secs(TIME_TRAIN).as_secs() {
		theta.0 -= LEARNING_RATE * (1.0 / (data.len() as f64)) * summation0(&data, theta) ;
		theta.1 -= LEARNING_RATE * (1.0 / (data.len() as f64)) * summation1(&data, theta); 
		if theta.0.is_nan() || theta.1.is_nan() {
			println!("<NAN FOUND !!>");
			break
		}
	}

	theta
}

fn main() {
	if env::args().count() != 2 {
		panic!("./train <filename or path of the filename>");
	}
	let mut rdr = ReaderBuilder::new()
					.from_path(env::args().last().expect("error from file"))
					.unwrap();
	let data : Vec<Vec<i32>> = rdr
				.records()
				.flatten()
				.map(|x| x.iter().map(|s| s.parse().unwrap()).collect())
				.collect();
	let result : (f64, f64) = train(data);

	println!("here for {} : {}", 100000, estimate_price(100000, result));
}
