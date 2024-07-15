use std::fs::File;
use std::io::ErrorKind;

fn main(){
/*     let greeting_file_result = File::open("hello.txt");
    let greet_file =  match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(file) => file,
                Err(error) => panic!("Couldn't create hello.txt: {}", error),
            },
            other_error => panic!("problem opening file: {other_error:?}"),
        }, 
    }; */
    // let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in the project");

}