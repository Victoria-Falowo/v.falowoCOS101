use std::io;

fn main() {
    let mut total:f32 = 0.0;

    println!("\t\tMenu\t\t\tPrice");
    println!("P = Poundo Yam/Edinkaiko Soup\t       -N3,200");
    println!("F = Fried Rice & Chicken\t       -N3,000");
    println!("A = Amala & Ewedu Soup\t       -N2,500");
    println!("E = Eba & Egusi Soup\t       -N2,000");
    println!("W = White Rice & Stew\t       -N2,500");

    println!("Enter letter of food you would like like to order (q to quit/ Submit order):");
    loop {
        let mut food = String::new();
        io::stdin().read_line(&mut food).expect("Failed to read input");
        let food = food.trim();

        if food == "P"{
            total += 3200.0;
        }else if food == "F"{
            total += 3000.0;
        }else if food == "A"{
            total += 2500.0;
        }else if food == "E"{
            total += 2000.0;
        }else if food == "W"{
            total += 2500.0;
        }else if food == "q"{
            break;
        }else {
            println!("Sorry, we don't have that here");
            continue;
        }

        println!("Your total is: N{}",total );
    }
    if total > 10_000.0{
        let total_discount:f32 = total - ((5.0 / 100.0) * total);
        println!("Total is over N10,000.\n 5% discount appplied \n New total is N{}",total_discount ); 
    }else {
        println!("Total: N{}",total );
    }
}
