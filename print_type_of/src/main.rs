fn main() {
    let var = "helloworld";
    print_type_of(&var);
}

fn print_type_of<T>(_: &T) //prints the type of any variable, use reference
{
    println!("{}", std::any::type_name::<T>())
}

