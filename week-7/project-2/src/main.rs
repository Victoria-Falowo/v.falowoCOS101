use std::io;

fn main() {
    let mut vt:Vec<String> = Vec::new();

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();

    println!("How many siblings do you have?");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let sib_no:u32 = input6.trim().parse().expect("Not a valid number");

    for i in 0..sib_no{
    println!("What is your siblings name?");
    io::stdin().read_line(&mut input9).expect("Not a valid string");
    let sib_name = input9.trim().expect("Not a valid string");
    vt.push(sib_name);

    println!("How old is sibling {}?", i+1);
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let sib_age:u32 = input1.trim().parse().expect("Not a valid number");
    vt.push(sib_age);

    if sib_age >= 18 {
        println!("Is your sibling single or married?");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        vt.push(input2);
        
        if input2 == "single"{
        println!("Is your sibling a student or worker?");
        io::stdin().read_line(&mut input3).expect("Not a valid string");
        vt.push(input3);

            if input3 == "student"{
                println!("What university are they in?");
                io::stdin().read_line(&mut input4).expect("Not a valid string");
                vt.push(input4)

                println!("What is their course of study?");
                io::stdin().read_line(&mut input5).expect("Not a valid string");
                vt.push(input5);

                println!("Your Sibling's Information \nAge: {} \nRelationship Status: {} \nOccupation: {} \nUniversity:{}\nCourse of Study{}",sib_age, restat[1],restat[3],input4,input5);
            }else if input3 == "worker"{
                println!("Noted!");
            }
        }else if input2 == "married"{

            println!("What city do they live in with their family?");
            io::stdin().read_line(&mut input8).expect("Not a valid string");
            vt.push(input8);

            println!("Do they have offspring(s)?\n (true for yes and false for no)");
            io::stdin().read_line(&mut input7).expect("Not a valid string");
            let off:bool = input7.trim().parse().expect("Failed to read input");
            vt.push(input7);

            if off == true{
                for i in vt {
                    println!("Your Sibling's Information \nName: {} \nAge: {} \nRelationship Status: {} \nOccupation: {} \nOffspring:{}\nCity:{}",vt, restat[0],restat[2],off,input8);
                }
            }

        }

    }else if sib_age < 18{ println!("How sad"); } } }
