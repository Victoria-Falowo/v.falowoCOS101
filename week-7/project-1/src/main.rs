use std::io;

fn main() {
    println!("What calculation would you like to run?");
    println!("1. Area of a trapezium \n2. Area of a rhombus \n3. Area of a Parallelogram \n4. Area of a cube \n5. Volume of a Cylinder");
    println!("Note: All input values for calculation should be written with a decimal point (If value is 5 then write as 5.0)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    let a:u32 = input.trim().parse().expect("Mot a valid number");
    
    if a == 1{
      trap_area();  
    }else if a == 2{
       rhom_area();
    }else if a == 3{
        para_area();
    }else if a == 4{
        cube_area();
    }else if a == 5{
        cyl_vol();
    }else {
        println!("We dont have that calculation");
    }
}

fn trap_area(){
    let mut tinput1 = String::new();
    let mut tinput2 = String::new();
    let mut tinput3 = String::new();

    println!("What is the height of the trapezium?");
    io::stdin().read_line(&mut tinput1).expect("Not a valid string");
    let height:f32 = tinput1.trim().parse().expect("Not a valid number");

    println!("What is the length of the first base?");
    io::stdin().read_line(&mut tinput2).expect("Not a valid string");
    let base1:f32 = tinput2.trim().parse().expect("Not a valid number");

    println!("What is the length of the second base?");
    io::stdin().read_line(&mut tinput3).expect("Not a valid string");
    let base2:f32 = tinput3.trim().parse().expect("Not a valid number");

    let a1 = height/(2.0 * (base1 + base2));
    println!("The area of the Trapezium is {}",a1);
}
fn rhom_area(){
    let mut rinput1 = String::new();
    let mut rinput2 = String::new();

    println!("What is the length of the first diagonal?");
    io::stdin().read_line(&mut rinput1).expect("Not a valid string");
    let d1:f32 = rinput1.trim().parse().expect("Not a valid number");

    println!("What is the length of the second diagonal?");
    io::stdin().read_line(&mut rinput2).expect("Not a valid string");
    let d2:f32 = rinput2.trim().parse().expect("Not a valid number");

    let a2 = 0.5 * d1 * d2;
    println!("The area of the rhombus is {}",a2);
}
fn para_area(){
    let mut pinput1 = String::new();
    let mut pinput2 = String::new();

    println!("What is the length of the base?");
    io::stdin().read_line(&mut pinput1).expect("Not a valid string");
    let b1:f32 = pinput1.trim().parse().expect("Not a valid number");

    println!("What is the altitude?");
    io::stdin().read_line(&mut pinput2).expect("Not a valid string");
    let alt:f32 = pinput2.trim().parse().expect("Not a valid number");

    let a3 = b1 * alt;
    println!("The area of the parallelogram is {}",a3);
}
fn cube_area(){
    let mut cinput1 = String::new();

    println!("Input the length of one side");
    io::stdin().read_line(&mut cinput1).expect("Not a valid string");
    let s:f32 = cinput1.trim().parse().expect("Not a valid number");

    let a4 = 6.0 * s.powf(2.0);

    println!("The area of the cube is {}",a4);
}
fn cyl_vol(){
   let pi:f32 = 3.142;
   let mut cyinput1 = String::new();
   let mut cyinput2 = String::new();

   println!("What is the radius?");
    io::stdin().read_line(&mut cyinput1).expect("Not a valid string");
    let rad:f32 = cyinput1.trim().parse().expect("Not a valid number");

    println!("What is the height?");
    io::stdin().read_line(&mut cyinput2).expect("Not a valid string");
    let cyh:f32 = cyinput2.trim().parse().expect("Not a valid number");

    let vol = pi * rad.powf(2.0) * cyh;

    println!("The volume of the cylinder is {}",vol);
}


