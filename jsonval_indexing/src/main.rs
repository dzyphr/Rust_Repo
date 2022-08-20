use serde_json::{Value};
use std::{collections::HashMap};
fn main() 
{
    let json = "[{\n\t\"json\":\n\t\t[{\"data\": 0}]\n}]";//here is a generic valid json object with one entry
    let json_obj: Vec<HashMap<String, Value>> = serde_json::from_str(&json).expect("error indexing into json object");
    //json_obj will contain HashMaps of (String, Value) pairs derived from the json object using serde_json 
    for (key, value) in &json_obj.clone()[0]//loop through each pair within a clone of the json object, specify index 0 for the vector
    {
        if key == "json"//comparing value of key with string
        {
            println!("key named 'json' found!");
            let val = &value[0]["data"];//finding the value of the "data" field within this key
            dbg!(val);
            let val_i64 = val.as_i64();//converting the json Number value to an i64 for computations
            dbg!(val_i64);
        }
    }
    dbg!(json_obj);
}
