use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    for y in 1..501 {
    println!("YOU ARE THE {}TH RESEARCHER",y );

    println!("Enter your full name");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    println!("How many papers have you published?");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let papers:u32 = input2.trim().parse().expect("Not a valid number");

        if papers > 3 && papers < 5 {
            println!("{} your incentive is N500,000",input1 );
        }else if papers > 5 && papers < 10 {
            println!("{} your incentive is N800,000",input1 );
        }else if papers >= 10 {
            println!("{} your incentive is N1,000,000",input1 );
        }else {
            println!("{} your incentive is N100,000",input1);
        }
    }
}
