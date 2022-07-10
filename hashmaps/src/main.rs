use std::collections::HashMap;
fn main() 
{
    let mut hashmap = HashMap::new();
    hashmap.insert("Data1", 256);
    hashmap.insert("Data2", 512);
    hashmap.insert("Data3", 1024);
    hashmap.insert("Data5", 2048);
    let number_of_entries = hashmap.len();
    match hashmap.get("Data1")
    {
        Some(data) => println!("Value of selected data:  {}\n", data),
        None => println!("No values are present for selected data\n")
    }
    
    for (var, data) in &hashmap
    {
        println!("Var:  {} Contains:    {}\n", var, data);
    }
}
