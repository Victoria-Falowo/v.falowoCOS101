//Rust programme that tells how fast a car is travelling in kilomoters from distance in miles

use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();

     println!("Enter distance travelled by car in miles:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let d:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter time taken to cover distance");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let t:f32 = input2.trim().parse().expect("Not a valid number");

    let speed = d *(1.61) / t;

    println!("Speed of the car: {}km/h", speed);


}
