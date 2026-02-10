use std::{fs::read_link, io::Read, str::Chars};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input);
    let token_tree = lexer(input);
    println!("{:#?}", token_tree);
}
fn lexer(raw: String) -> Vec<Token> {
    let iter: Vec<char> = raw.chars().collect();
    let mut index = 0;
    let mut result = Vec::new();
    loop {
        let mut keyword = String::new();
        if peek(&iter, &mut index, "함수마당 ") {
            result.push(Token::Function);
            continue;
        } else if peek(&iter, &mut index, "<") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::TriangleOpen);
            continue;
        } else if peek(&iter, &mut index, ">") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::TriangleClose);
            continue;
        } else if peek(&iter, &mut index, "->") {
            result.push(Token::ReturnType);
            continue;
        } else if peek(&iter, &mut index, "장군님께_뜻_올려드리기 ") {
            result.push(Token::Return);
            continue;
        } else if peek(&iter, &mut index, " ") || peek(&iter, &mut index, "\n") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::WhiteSpace);
            continue;
        } else if peek(&iter, &mut index, ";") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::SemiColumn);
            continue;
        } else if peek(&iter, &mut index, "(") {
            flush_keyword(&mut keyword, &mut result);
        } else if peek(&iter, &mut index, "\"") {
            let mut string = String::new();
            loop {
                if iter.get(index) == Some(&'\"') {
                    index += 1;
                    break;
                } else if iter.get(index) == Some(&'\\') {
                    todo!()
                } else if let Some(ch) = iter.get(index) {
                    string.push(*ch);
                } else {
                    panic!("동무! 코딩좀 똑바로 하라우!");
                }
                index += 1;
            }
            result.push(Token::Str(string));
            continue;
        } else if let Some(ch) = iter.get(index) {
            keyword.push(*ch);
            index += 1;
            continue;
        } else {
            return result;
        }
    }
}
fn flush_keyword(buffer: &mut String, result: &mut Vec<Token>) {
    if !buffer.is_empty() {
        result.push(Token::KeyWord(std::mem::take(buffer)));
    }
}
#[derive(Debug)]
enum Token {
    Function,
    TriangleOpen,
    TriangleClose,
    KeyWord(String),
    WhiteSpace,
    ReturnType,
    GoalHoOpen,
    GoalHoClose,
    Return,
    SemiColumn,
    Str(String),
    Ignore(char),
}
fn peek(array: &Vec<char>, index: &mut usize, str: &str) -> bool {
    let mut index_failback = index.clone();
    for i in str.chars() {
        if array.get(*index) == Some(&i) {
            *index += 1;
            continue;
        } else {
            *index = index_failback;
            return false;
        }
    }
    true
}
