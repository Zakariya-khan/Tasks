use std::cmp::Ordering;
fn main() {
    println!("Please Enter Your Marks");
    
    let mut marks = String::new();
    std::io::stdin()
    .read_line(&mut marks)
    .expect("Failed to read line");
    let marks : i32 = marks.trim().parse().expect("Please Enter Valid Marks");

    if marks >= 1100{
        println!("Invalid Marks");
    }else if marks < 1100 && marks >= 950 {
        println!("Congo! You got A+ Grade");
    }else if marks <= 950 && marks >= 800 {
        println!("Great! You got A Grade");
    }else if marks <= 800 && marks >= 650{
        println!("Keep It Up! You got B Grade");
    }else if marks <= 650 && marks >= 500 {
        println!("You got C Geade");
    }else if marks <= 500 && marks >= 350{
        println!("You got D Grade");
    }else{
        println!("Failed!");
    }
    
    percentage(marks);
}

fn percentage(c : i32){
    if c < 1100{
    let float = c as f64 ;
    let percentage = float / 1100.0 * 100.0;
    println!("Your percentage is {}",percentage );
    }else{
        println!("Failed to find your percentage");
    }
}