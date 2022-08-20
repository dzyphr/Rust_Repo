use crate::Cases::{Red, Green, Blue, Yellow};//import the enum for quick typed referencing
#[derive(Debug)]
enum Cases//Define expected cases
{
    Red,
    Green,
    Blue,
    Yellow
}

fn main() 
{
    process_enum(Red);
    process_enum(Green);
    process_enum(Blue);
    process_enum(Yellow);
}

fn process_enum(case: Cases)//pass a case to this function to get matched
{
    match case
    {
        Red => println!("case is red!"),
        Green => println!("case is green!"),
        Blue => println!("case is blue!"),
        Yellow => println!("case is yellow!")
    }
}
