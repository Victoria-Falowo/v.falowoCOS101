//Student council voting system

use std::io;

fn main() {
    let mut name = String::new();
    let mut email = String::new();
    let mut input = String::new();
    let mut department = String::new();
    let mut state_of_origin = String::new();
    let mut level = String::new();
    let mut class_rep = String::new();

    for y in 1..151 {
    println!("YOU ARE THE {}th CANDIDATE",y );

    println!("Enter your fullname");
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Enter your email (preferably school email)");
    io::stdin().read_line(&mut email).expect("Failed to read input");

    println!("Enter your department");
    io::stdin().read_line(&mut department).expect("Failed to read input");

    println!("Are you class rep (Y/N)");
    io::stdin().read_line(&mut class_rep).expect("Failed to read input");

    println!("Enter your state of origin ");
    io::stdin().read_line(&mut state_of_origin).expect("Failed to read input");

    println!("Enter your CGPA");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let cpga:f32 = input.trim().parse().expect("Not a valid number");

    println!("Enter your level of study");
    io::stdin().read_line(&mut level).expect("Failed to read input");
    let l:u32 = level.trim().parse().expect(" Not a valid number");

    if class_rep == "Y" && l > 100 && cpga >= 4.0 {
        println!("{}\n{}\n{}\n{}\n",name, email, department, state_of_origin );

        println!("You can vote");
    }else {
        println!("Sorry, you are not eligible to vote");
    }
    }

}
