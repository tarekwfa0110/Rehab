use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to readline");
    let number: i32 = input.trim().parse().expect("This is not a number!!");
    if number != 2 && number%2 == 0{
        println!("YES");
    }
    else{
        println!("NO");
    }
}