use rand::Rng;
use std::cmp::Ordering;

fn main(){  
  for counter in (1..4).rev(){
  println!("Please Guess number \n{} Attemps left",counter  );
  let mut input = String::new();
  std::io::stdin()
  .read_line(&mut input)
  .expect("failed to read line");
  let input :u32 = match input.trim().parse(){
    Ok(num) => num,
    Err(_) => continue,
  };

  println!("You guessed: {}",input);

  let secret_number = rand::thread_rng().gen_range(1, 10);
  println!("Secret number is {}",secret_number );

  match input.cmp(&secret_number) {
    Ordering::Less => println!("Your answer is Small \nTry again"),
    Ordering::Greater => println!("Your answer is Greater \nTry again "),
    Ordering::Equal => println!("Congo! You Win"),
  }
  if input == secret_number{
      break
  }
 }
}

