use binary_search::logic::dict::dict;
use binary_search::logic::calculator;
use binary_search::lexicon_buddy;


fn main() {
    match lexicon_buddy::run() {
        Ok(addr) => {
            println!("Addresses for it: {addr}");
        }
        Err(error) => match error {
            dict::DictError::IOError(e) => println!("IO error! {e}"),
            dict::DictError::ParseError(e) => println!("ParseError {e}"),
            dict::DictError::ParseIntError(e) => println!("ParseError {e}"),
            dict::DictError::WordNotFound => println!("Word not fond"),
        },
    };
    let mut context = calculator::Context::new();
    let result = context.calculate("10");
    println!("reslut is {result}")
}
