use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    //panic!("crash and burn");

    //let v = vec![1, 2, 3];
    //v[99];

    let f = File::open("hello.txt");
    //let g = File::open("hello2.txt").unwrap();
    //let h = File::open("hello3.txt").expect("Failed to open hello3.txt");

    /*
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem optioning the file: {:?}", other_error),
        },
    };
    */

    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem optioning the file: {:?}", error);
        }
    });

    let u = read_username_from_file();
    println!("{:?}", u);
}

/*
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
*/

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}