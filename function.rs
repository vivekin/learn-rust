fn alpha(x: i32)
{
   println!("inside fn alpha, value of x is: {x}"); 
}

fn alpha(x: i32)-> ()
{
   println!("inside fn alpha, value of x is: {x}"); 
}

fn five() -> i32 {
    5
}

fn add(a: i32, b: i32) -> i32 {
  // return a + b;
  // return a + b
  a + b // No semicolon means this value is returned
}

fn main() {
    alpha(61);

    let x = five();
    println!("value of five is {x}");
}
