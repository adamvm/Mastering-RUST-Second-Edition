
fn main() {
    let request_code = 200;
    match request_code {

        200 => println!("Success"),

        404 => println!("Not Found"),
        
        // can also use match guards
        code if (code > 400 || code <= 500) => println!("Server Error {}", code),

        error => {
            // multiline match arm
            let error_str = format!("Network error! Reason: {}", error);
            println!("{}", error_str);
        }

    }
}
