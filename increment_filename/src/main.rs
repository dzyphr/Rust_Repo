use std::{io::Write, fs::File};
fn main() 
{
    let loopamount = 2;//will make loopamount # of files, 1 for each loop
    for i in 1..loopamount+1//will set i to 1 to start (file1.txt) add 1 to loop otherwise it will stop at 7 instead of loop through it
    {
        let filename = "file".to_string().to_owned() + &i.to_string() + &".txt".to_string();//filname takes i every loop
        let mut file = File::create(filename).expect("error setting up filehandle");//use a create file. assumes it doesnt exist yet
        let data = "this will be written into the file";//any string data can be written as bytes
        file.write_all(data.as_bytes()).expect("error writing data to file");//write all the data as bytes
    }
}
