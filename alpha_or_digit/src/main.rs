fn main() 
{//runs through some example characters checks against useful common character types
    let mut chosen_char = '1';
    println!("Is {} a decimal digit?\n{}",chosen_char,  chosen_char.is_digit(10));
    println!("Is {} alphanumeric?\n{}", chosen_char, chosen_char.is_alphanumeric());
    println!("Is {} a hexadecimal?\n{}", chosen_char, chosen_char.is_digit(16));
    println!("Is {} alphabetical?\n{}", chosen_char, chosen_char.is_alphabetic());
    chosen_char = 'A';
    println!("Is {} a decimal digit?\n{}",chosen_char,  chosen_char.is_digit(10));
    println!("Is {} alphanumeric?\n{}", chosen_char, chosen_char.is_alphanumeric());
    println!("Is {} a hexadecimal?\n{}", chosen_char, chosen_char.is_digit(16));
    println!("Is {} alphabetical?\n{}", chosen_char, chosen_char.is_alphabetic());
    chosen_char = 'G';
    println!("Is {} a decimal digit?\n{}",chosen_char,  chosen_char.is_digit(10));
    println!("Is {} alphanumeric?\n{}", chosen_char, chosen_char.is_alphanumeric());
    println!("Is {} a hexadecimal?\n{}", chosen_char, chosen_char.is_digit(16));
    println!("Is {} alphabetical?\n{}", chosen_char, chosen_char.is_alphabetic());
}

