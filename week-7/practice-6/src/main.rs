fn main() {
    let mut num:i32 = 5;
    mutate_num_to_zero(&mut num);
    println!("The value of num is: {}",num) ;
    // if mutate_num_to_zero comes before println then 5 and ten are printed, if not then 10 is printed for both
}

fn mutate_num_to_zero(param_num:&mut i32){
    *param_num = *param_num*2; //de reference
    println!("param_num value is: {}",param_num );
}
