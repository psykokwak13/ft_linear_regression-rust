use std::{
    fs::File,
    io::{BufReader, Write},
};

const DATA_THETA_FILE: &str = "data_theta.json";

fn estimate_price(mileage: i32, theta: (f64, f64)) -> f64 {
    theta.0 + theta.1 * (mileage as f64)
}

// collect datas from our json
fn get_data_from_json(reset: bool) -> (f64, f64) {
    let mut file = File::open(DATA_THETA_FILE).expect("error while opening file.");

    if reset == true {
        file.write_all("".as_bytes()).expect("error with file.");
    }

    let buf = BufReader::new(file);

    serde_json::from_reader(buf).expect("error with json.")
}

fn main() {
    let mut buffer: String = String::new();
    let mut reset_buffer: String = String::new();

    println!("Guess the price with mileage of the car : ");
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("error reading stdin.");

    // asking if we want to reset training
    println!("reset training after prediction ? (y/n) : ");
    let mut reset: bool = false;
    std::io::stdin()
        .read_line(&mut reset_buffer)
        .expect("error reading stdin.");

    if reset_buffer.trim() == "y" {
        reset = true;
    }

    println!(
        "result : {}",
        estimate_price(
            buffer.trim().parse::<i32>().expect("error in parse."),
            get_data_from_json(reset)
        )
    );
}
