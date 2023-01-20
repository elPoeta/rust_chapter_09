#![allow(unused)]

use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};
/*
enum Result<T, E> {
    Ok(T),
    Err(E),
} */
fn main() {
    {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }
    {
        // Matching on Different Errors

        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
    }
    {
        // Alternatives to Using match with Result<T, E>

        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    {
        // Shortcuts for Panic on Error: unwrap and expect

        let greeting_file = File::open("hello.txt").unwrap();

        //  Using expect instead of unwrap and providing good error messages

        let greeting_file =
            File::open("hello.txt").expect("hello.txt should be included in this project");
    }
    {
        // Propagating Errors
        fn read_username_from_file() -> Result<String, io::Error> {
            let username_file_result = File::open("hello.txt");

            let mut username_file = match username_file_result {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut username = String::new();

            match username_file.read_to_string(&mut username) {
                Ok(_) => Ok(username),
                Err(e) => Err(e),
            }
        }
    }

    {
        // A Shortcut for Propagating Errors: the ? Operator

        fn read_username_from_file() -> Result<String, io::Error> {
            let mut username_file = File::open("hello.txt")?;
            let mut username = String::new();
            username_file.read_to_string(&mut username)?;
            Ok(username)
        }

        // chaining method

        fn read_username_from_file2() -> Result<String, io::Error> {
            let mut username = String::new();

            File::open("hello.txt")?.read_to_string(&mut username)?;

            Ok(username)
        }
    }
    {
        assert_eq!(
            last_char_of_first_line("Hello, world\nHow are you today?"),
            Some('d')
        );

        assert_eq!(last_char_of_first_line(""), None);
        assert_eq!(last_char_of_first_line("\nhi"), None);

        fn last_char_of_first_line(text: &str) -> Option<char> {
            text.lines().next()?.chars().last()
        }
    }
}

/*
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
*/
