use std::io::Read;
use std::io;

fn main() {
    let mut input1 = String::new();
    let num1 = 1;

    println!("What is your role in the company \n1. Administrator \n2. Project Manager \n3. Employee \n4. Customer \n5. Vendor \n(Input one of the corresponding numbers)");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let ad1:i32 = input1.trim().parse().expect("Not a valid number");

    if ad1 == num1{
    let mut file = std::fs::File::open("globalcom_db.sql").unwrap();
    let mut contents1 = String::new();
    file.read_to_string(&mut contents1).unwrap();
    print!("{}",contents1); 
    }else {
        project(ad1);
    }
}
fn project(mut ad2:i32){
    let num2 = 2;
    ad2 = ad2*1;

    if ad2 == num2{
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents1 = String::new();
    file.read_to_string(&mut contents1).unwrap();
    print!("{}",contents1); 
    }else {
        employee(ad2);
    }
}

fn employee(mut ad3:i32){
    let num3 = 3;
    ad3 = ad3*1;

    if ad3 == num3{
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents1 = String::new();
    file.read_to_string(&mut contents1).unwrap();
    print!("{}",contents1); 
    }else {
        customer(ad3);
    }
}

fn customer(mut ad4:i32){
    let num4 = 4;
    ad4 = ad4*1;

    if ad4 == num4{
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents1 = String::new();
    file.read_to_string(&mut contents1).unwrap();
    print!("{}",contents1); 
    }else {
        vendor(ad4);
    }
}

fn vendor(mut ad5:i32){
    let num5 = 5;
    ad5 = ad5*1;

    if ad5 == num5{
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents1 = String::new();
    file.read_to_string(&mut contents1).unwrap();
    print!("{}",contents1); 
    }else {
        println!("You do not have access to the database");
    }  
}
