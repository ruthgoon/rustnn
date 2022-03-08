use std::io;
use std::process;
use rand::{Rng, prelude::ThreadRng};
mod dataloader;
mod model;

/// Returns a new 2D weight vector
/// x should be the dimensionality of the vector, with y being the number of classes the vector 
/// has. The value returned is a 2D vector filled with random floats from 0 to 1.
/// Note, Vec<f32> does not support .map() and iterators so looping is the only way to do this
pub fn init_random_weights(x : usize, y: usize, rng: ThreadRng) -> Vec<Vec<f32>>{
    let mut rng = rand::thread_rng();
    let mut random_weights = vec![vec![0.0;x];y];
    for i in 0..x{
        for j in 0..y{
            random_weights[i][j] = rng.gen::<f32>();
        }
    }
    return random_weights;
} 


/// Main Function
/// Invokes inferencing or the main training loop
fn main() {
    let mut user_input = String::new();
    let allowed_vals = ["train","inference","quit"];
    let mut rng = rand::thread_rng();
    let training_file = String::from("./data/train-images-idx3-ubyte");
    let label_file = String::from("./data/train-images-idx3-ubyte");
     // load the MNIST data as a Dataset struct
    println!("Loading MNIST dataset (might take a while)...");
    let mnist_dataset = dataloader::read_mnist_training_set(&label_file, &training_file);
    println!("Loaded!\nEnter in command to continue\n[train\tinference\tquit]");
    let mut captive_input = true;

    // user input stuff
    while captive_input{
        user_input.clear();
        io::stdin().read_line(&mut user_input).unwrap();
        user_input = user_input.trim().to_string();
        
        for v in allowed_vals{
            if user_input == v{
                captive_input = false;
            }
        }
        if captive_input{
            println!("Error! Command not recognized");
        }
    }

    if user_input == "quit"{
        process::exit(0);
    }
    
    // MODEL INFERENCING
    if user_input == "inference"{
        println!("Not supported yet!");
        process::exit(0);
    }
    
    // MODEL TRAINING
    println!("Initializing random weights & bias vectors...");
    

}
