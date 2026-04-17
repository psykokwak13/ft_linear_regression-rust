# ft_linear_regression-rust
A simple linear regression implementation in Rust that predicts car prices based on mileage.
Train a model using gradient descent on a dataset, then saves the learned parameters to a JSON file.

⚠️ reset training still don't work

Project made with Cargo.
# How to use
### Train the model :
Use the following command :
```bash
cargo run --bin train <datafile>
```
### Predict the price :
```bash
cargo run --bin predict
```
### Change training duration and learning rate :
Edit the constants in `/src/train/train.rs`
# Author
egatien 
