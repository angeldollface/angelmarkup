/*
ANGEL MARKUP LANGUAGE by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

use std::fmt;
use regex::Regex;
use std::fs::File;
use std::fs::write;
use colored::Colorize;
use std::error::Error;
use std::fs::read_to_string;
use std::collections::HashMap;
use serde_json::to_string_pretty;

/// Converts a HashMap into a YAML string.
pub fn to_yaml(subject:HashMap<String, String>) -> String {
    let mut result_list: Vec<String> = Vec::new();
    for (key, value) in subject.into_iter() {
        let code_string: String = format!("{}: \"{}\"", key, value);
        result_list.push(code_string);
    }
    let result: String = result_list.join("\n");
    return result;
}

/// Converts a HashMap into a TOML string.
pub fn to_toml(subject:HashMap<String, String>) -> String {
    let mut result_list: Vec<String> = Vec::new();
    for (key, value) in subject.into_iter() {
        let code_string: String = format!("{} = \"{}\"", key, value);
        result_list.push(code_string);
    }
    let result: String = result_list.join("\n");
    return result;
}

/// Converts a HashMap into a Angelmarkup string.
pub fn to_aml(subject:HashMap<String, String>) -> String {
    let mut result_list: Vec<String> = Vec::new();
    for (key, value) in subject.into_iter() {
        let code_string: String = format!("\'{}\' => \'{}\'", key, value);
        result_list.push(code_string);
    }
    let result: String = result_list.join("\n");
    return result;
}

// Checks whether a file exists and
/// returns a boolean to that effect.
pub fn file_is(filename: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let contents = read_to_string(filename);
    match contents {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
}

/// Tries to read a file and return
/// its contents.
pub fn read_file(filename: String) -> String {
    let mut result: String = String::from("");
    let fname_copy: String = filename.clone();
    if file_is(filename) == true {
        result = read_to_string(fname_copy).unwrap();
    }
    else {}
    return result;
}

/// Checks if "subject" has the index "index".
pub fn has_index(subject: Vec<Token>, index: usize) -> bool {
    let mut result: bool = false;
    if index >= subject.len(){
        result = true;
    }
    else {}
    return result;
}

/// A struct to store and retrieve data
/// about all lexed tokens.
#[derive(Clone,Eq,PartialEq)]
pub struct Token {
    name: String,
    value: String
}

/// Populates the [Token] struct with
/// empty values for easier use.
impl Default for Token {
    fn default () -> Token {
        Token {
            name: String::from(""),
            value: String::from("")
        }
    }
}

impl Token {
    pub fn to_string(&self) -> String {
        return format!("{} : {}", self.name, self.value);
    }
}

/// A [HashMap] for tokens the lexer recognises.
pub fn pattern_pool() -> HashMap<String, Regex>{
    let mut pool: HashMap<String, Regex> = HashMap::new();
    pool.insert(String::from("ENTITY"), Regex::new(r"'(.*)'").unwrap());
    pool.insert(String::from("ASSIGN"), Regex::new(r"(=>)").unwrap());
    pool.insert(String::from("COMMENT"), Regex::new(r"#(.*)").unwrap());
    return pool;
}

/// The actual lexing function: Iterates through all lines
/// and then through all characters and builds a vector of tokens
/// while doing so and finally returns this vector.
pub fn lex(source_code: String) -> Vec<Token>{
    let lines: Vec<String> = clean_split(source_code, String::from("\n"));
    let mut result: Vec<Token> = Vec::new();
    let pool: HashMap<String, Regex> = pattern_pool();
    for line in lines {
        let char_list: Vec<String> = clean_split(line, String::from(""));
        let mut new_char_list: Vec<String> = Vec::new();
        for char_item in char_list {
            new_char_list.push(char_item);
            let collected_chars: String = new_char_list.join("");
            for (key,value) in pool.clone().into_iter() {
                if value.is_match(&collected_chars) {
                    new_char_list.clear();
                    let captured = value.captures(&collected_chars).unwrap();
                    let new_token: Token = Token {
                        name: key,
                        value: captured.get(1).unwrap().as_str().to_string()
                    };
                    result.push(new_token);
                }
                else {}
            }
        }
    }
    return result;
}

// Returns a vector of strings from a character split for a string.
/// Both the string and split character have to be strings.
pub fn clean_split(subject: String, split_char: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in subject.split(&split_char) {
        let new_item: String = item.to_string();
        result.push(new_item);
    }
    return result;
}

/// An error struct
/// to catch Angelmarkup
/// errors.
#[derive(Debug)]
pub struct AngelMarkupError {
    details: String
}

/// Implements a method to instantiate the
/// struct.
impl AngelMarkupError {
    fn new(msg: &str) -> AngelMarkupError {
        AngelMarkupError{details: msg.to_string()}
    }
}

/// Implements stuff for better display.
impl fmt::Display for AngelMarkupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

/// Implements the error trait.
impl Error for AngelMarkupError {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Serializes an AML string into a "HashMap".
pub fn serialize(src: String) -> Result<HashMap<String, String>,AngelMarkupError> {
    let lexed_tokens: Vec<Token> = lex(src);
    let lexed_tokens_clone_one: Vec<Token> = lexed_tokens.clone();
    let lexed_tokens_clone_two: Vec<Token> = lexed_tokens_clone_one.clone();
    let lexed_tokens_clone_three: Vec<Token> = lexed_tokens_clone_two.clone();
    let lexed_tokens_clone_four: Vec<Token> =  lexed_tokens_clone_three.clone();
    let lexed_tokens_clone_five: Vec<Token> =  lexed_tokens_clone_four.clone();
    let lexed_tokens_clone_six: Vec<Token> =  lexed_tokens_clone_five.clone();
    let mut result: HashMap<String, String> = HashMap::new();
    for (index,token) in lexed_tokens.into_iter().enumerate() {
        if token.name == String::from("ASSIGN"){
            let last_index: usize = index-1;
            let next_index: usize = index+1;
            if last_index < lexed_tokens_clone_three.len() && next_index < lexed_tokens_clone_four.len() {
                let key: String = lexed_tokens_clone_five[last_index.clone()].clone().value;
                let value: String = lexed_tokens_clone_six[next_index.clone()].clone().value;
                result.insert(key,value);
            }
            else {
                let msg: String = String::from("Syntax error detected!");
                return Err(AngelMarkupError::new(&msg));
            }
        }
        else if token.name == String::from("COMMENT") {}
        else {}
    }
    if result.is_empty(){
        let msg: String = String::from("Result is empty.");
        return Err(AngelMarkupError::new(&msg));
    }
    else {}
    Ok(result)
}

/// Lints your AML code.
/// If everything is A-OK,
/// "true" is returned.
pub fn lint(src: String) -> bool {
    let mut result: bool = false;
    let match_op = serialize(src);
    match match_op {
        Ok(_x) => {
            result = true;
        },
        Err(_e) => {}
    };
    return result;
}

// Tries to create a file and returns
/// a boolean depending on whether the
/// operation succeeded.
pub fn create_file(filename: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let new_file = File::create(filename);
    match new_file {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
}

/// Tries to write to a file and returns
/// a boolean depending on whether the
/// operation succeeded.
pub fn write_to_file(filename: String, contents: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let fname_copy: String = filename.clone();
    if file_is(filename) == true {
        let write_op = write(fname_copy, contents);
        match write_op {
            Ok(_n) => result.push(true),
            Err(_x) => result.push(false)
        }
    }
    return result[0];
}

/// Compiles an AML file to a JSON file.
pub fn compile_to_json(src: String, target: String) {
    let src_clone_one: String = src.clone();
    let src_clone_two: String = src_clone_one.clone();
    let target_clone_one: String = target.clone();
    let target_clone_two: String = target_clone_one.clone();
    if lint(read_file(src_clone_one)) == true {
        let json_string: String = to_string_pretty(&serialize(read_file(src_clone_two)).unwrap()).unwrap();
        create_file(target_clone_one);
        write_to_file(target_clone_two, json_string);
    }
    else {
        let msg: String = format!("An error occurred while parsing your Angelmarkup file.").red().to_string();
        println!("{}", msg);
    }
}

/// Compiles an AML file to a YAML file.
pub fn compile_to_yaml(src: String, target: String) {
    let src_clone_one: String = src.clone();
    let src_clone_two: String = src_clone_one.clone();
    let target_clone_one: String = target.clone();
    let target_clone_two: String = target_clone_one.clone();
    if lint(read_file(src_clone_one)) == true {
        let yml_string: String = to_yaml(serialize(read_file(src_clone_two)).unwrap());
        create_file(target_clone_one);
        write_to_file(target_clone_two, yml_string);
    }
    else {
        let msg: String = format!("An error occurred while parsing your Angelmarkup file.").red().to_string();
        println!("{}", msg);
    }
}

/// Compiles an AML file to a TOML file.
pub fn compile_to_toml(src: String, target: String) {
    let src_clone_one: String = src.clone();
    let src_clone_two: String = src_clone_one.clone();
    let target_clone_one: String = target.clone();
    let target_clone_two: String = target_clone_one.clone();
    if lint(read_file(src_clone_one)) == true {
        let toml_string: String = to_toml(serialize(read_file(src_clone_two)).unwrap());
        create_file(target_clone_one);
        write_to_file(target_clone_two, toml_string);
    }
    else {
        let msg: String = format!("An error occurred while parsing your Angelmarkup file.").red().to_string();
        println!("{}", msg);
    }
}
