use kalk::parser;
use std::fmt::Error;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::ops::Deref;



pub struct Context(parser::Context);

impl Deref for Context {
    type Target = parser::Context;
    fn deref(&self) -> &parser::Context {
        &self.0 // 返回元组结构体的第一个元素的引用
    }
}
pub fn get_display_string(input: &str) -> String {
    let mut output = String::from(input);
    for (key, value) in REPLACEMENTS.iter() {
        match input.find(key){
            Some(offset) => output.replace_range(offset..offset+key.len(),value),
            None => (),
        }
    }
    output
}

impl Context{
    pub fn new() -> Context {
        Context(parser::Context::new())
    }

    pub fn calculate(mut self, input: &str) -> String {
        match parser::eval(&mut self.0, input) {
            Ok(Some(mut result)) => {
                result.to_string()
                //result.to_string_pretty()
            }
            Ok(None) => String::from(""),
            Err(err) => err.to_string(),
        }
    }
}


lazy_static! {
    pub static ref REPLACEMENTS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("ceil", "⌈⌉");
        m.insert("deg", "°");
        m.insert("floor", "⌊⌋");
        m.insert("gamma", "Γ");
        m.insert("sum", "Σ()");
        m.insert("prod", "∏()");
        m.insert("integrate", "∫()");
        m.insert("integral", "∫()");
        m.insert("phi", "ϕ");
        m.insert("pi", "π");
        m.insert("sqrt", "√");
        m.insert("tau", "τ");
        m.insert("(", "()");
        m.insert("[[", "⟦⟧");
        m.insert("!=", "≠");
        m.insert(">=", "≥");
        m.insert("<=", "≤");
        m.insert(" and", " ∧");
        m.insert(" or", " ∨");
        m.insert(" not", " ¬");
        m.insert("*", "×");
        m.insert("/", "÷");
        m.insert("^T", "ᵀ");
        m.insert("asin", "sin⁻¹()");
        m.insert("acos", "cos⁻¹()");
        m.insert("atan", "tan⁻¹()");
        m.insert("acot", "cot⁻¹()");
        m.insert("acosec", "cosec⁻¹()");
        m.insert("asec", "sec⁻¹()");
        m.insert("asinh", "sinh⁻¹()");
        m.insert("acosh", "cosh⁻¹()");
        m.insert("atanh", "tanh⁻¹()");
        m.insert("acoth", "coth⁻¹()");
        m.insert("acosech", "cosech⁻¹()");
        m.insert("asech", "sech⁻¹()");
        m.insert("cbrt", "∛");
        m
    };
}


// struct LbCalculationResult{
//     float:f64,
//     string:String,
//     string_pretty:String,
//     string_big:String,
//
// }



