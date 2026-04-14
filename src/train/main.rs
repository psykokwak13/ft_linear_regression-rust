use csv::ReaderBuilder;
use std::{env, time::{Duration, Instant}};

const TIME_TRAIN : u64 = 10;
const LEARNING_RATE : f32 = 0.01;

fn estimate_price(mileage : i32, theta : (f32, f32)) -> f32{
	theta.0 + theta.1 * (mileage as f32)
}

fn summation0(data : &Vec<Vec<i32>>, theta : (f32, f32)) -> f32 {
	let mut result = 0.0f32;

	for d in data.iter() {
		result += estimate_price(*(d.get(0).expect("number not found")), theta) - *(d.get(1).expect("number not found")) as f32;
	}

	result
}

fn summation1(data : &Vec<Vec<i32>>, theta : (f32, f32)) -> f32 {
	let mut result = 0.0f32;

	for d in data.iter() {
		result += (estimate_price(*(d.get(0).expect("number not found")), theta) - *(d.get(1).expect("number not found")) as f32) * *(d.get(0).expect("number not found")) as f32;
	}

	result
}

fn train(data : Vec<Vec<i32>>) -> (f32, f32) {
	let mut theta : (f32, f32) = (0.0, 0.0); // theta0, theta1 = 0.0

	let mut now = Instant::now();
	while now.elapsed().as_secs() != Duration::from_secs(TIME_TRAIN).as_secs() {
		theta.0 -= LEARNING_RATE * (1.0 / (data.len() as f32)) * summation0(&data, theta) ;
		theta.1 -= LEARNING_RATE * (1.0 / (data.len() as f32)) * summation1(&data, theta); 
	}

	println!("0 : {}, 1 : {}", theta.0, theta.1);
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

	let mut result : (f32, f32) = train(data);

	println!("voila pour {} : {}", 100000, estimate_price(100000, result));
}
