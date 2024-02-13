use binary_search::logic::dict::dict;
use binary_search::logic::calculator;


fn main() {

    let mut context = calculator::Context::new();
    let result = context.calculate("10*12.3*pi*asd");
    let disp = calculator::get_display_string("10*12.3*pi*asd");
    println!("reslut is {result}, disp {disp}")
}
