fn main() {
    let num:i32 = 5;
    mutate_num_to_zero(num);
    println!("The value of no is:{}", num);
}

fn mutate_num_to_zero(mut param_num: i32){
    param_num = param_num*2 ;
    println!("param_num values is :{}",param_num);
}
