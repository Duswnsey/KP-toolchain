use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input);
    let token_tree = lexer(input);
    println!("{:#?}", token_tree);
    let mut index = 0;
    let ast = parser(&token_tree, &mut index, true);
    println!("{:#?}", ast);
}
fn parser(token_tree: &Vec<Token>, index: &mut usize, top: bool) -> Vec<Element> {
    let mut result: Ast = Vec::new();
    while let Some(token) = token_tree.get(*index) {
        *index += 1;
        match token {
            Token::Function => {
                let name = if let Some(Token::KeyWord(name)) = token_tree.get(*index) {
                    name
                } else {
                    panic!();
                };
                *index += 1;
                if token_tree.get(*index) != Some(&Token::GoalHoOpen) {
                    panic!();
                }
                *index += 1;
                let mut arguments = Vec::with_capacity(5); //Heap
                loop {
                    if Some(&Token::GoalHoClose) == token_tree.get(*index) {
                        *index += 1;
                        break;
                    }
                    let name = if let Some(Token::KeyWord(name)) = token_tree.get(*index) {
                        name
                    } else {
                        panic!();
                    };
                    *index += 1;
                    if token_tree.get(*index) != Some(&Token::Column) {
                        panic!();
                    }
                    *index += 1;
                    let typee = get_type(&token_tree, index, 0);
                    arguments.push((name.clone(), typee));
                    if Some(&Token::GoalHoClose) == token_tree.get(*index) {
                        *index += 1;
                        break;
                    } else if Some(&Token::Comma) != token_tree.get(*index) {
                        panic!();
                    }
                    *index += 1;
                }
                let return_type = if token_tree.get(*index) == Some(&Token::ReturnType) {
                    *index += 1;
                    Some(get_type(&token_tree, index, 0))
                } else {
                    None
                };
                let scope = if token_tree.get(*index) == Some(&Token::MiddleGoalHoOpen) {
                    *index += 1;
                    parser(&token_tree, index, false)
                } else {
                    panic!();
                };
                result.push(Element::Function(Function {
                    name: name.to_string(),
                    argument: arguments,
                    return_type: return_type,
                    code: scope,
                }));
            }
            Token::MiddleGoalHoClose => {
                if !top {
                    *index += 1;
                    return result;
                }
            }
            _ => {}
        }
    }
    if !top {
        panic!("동무한테 실망했다우");
    }
    result
}
fn get_type(tree: &Vec<Token>, index: &mut usize, offset: usize) -> Type {
    let mut result;
    match tree.get(*index + offset) {
        Some(Token::KeyWord(name)) => {
            result = Type::Other(name.to_string());
            *index += 1;
        }
        Some(Token::GoalHoOpen) => {
            *index += 1;
            result = Type::Tuple(Vec::new());
            loop {
                if let Type::Tuple(ref mut tup) = result {
                    tup.push(get_type(tree, index, 0));
                }
                match tree.get(*index + offset) {
                    Some(Token::GoalHoClose) => {
                        *index += 1;
                        break;
                    }
                    Some(Token::Comma) => {
                        *index += 1;
                        continue;
                    }
                    _ => {
                        panic!()
                    }
                }
            }
        }
        _ => {
            panic!()
        }
    }
    result
}
type Ast = Vec<Element>;
#[derive(Debug)]
enum Element {
    Function(Function),
}
#[derive(Debug)]
enum Type {
    Tuple(Vec<Type>),
    Other(String),
}
#[derive(Debug)]
struct Function {
    argument: Vec<(String, Type)>,
    name: String,
    code: Ast,
    return_type: Option<Type>,
}
fn lexer(raw: String) -> Vec<Token> {
    let iter: Vec<char> = raw.chars().collect();
    let mut index = 0;
    let mut result = Vec::new();
    let mut keyword = String::new();
    loop {
        if peek(&iter, &mut index, "<") {
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
        } else if peek(&iter, &mut index, " ") || peek(&iter, &mut index, "\n") {
            flush_keyword(&mut keyword, &mut result);
            continue;
        } else if peek(&iter, &mut index, ";") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::SemiColumn);
            continue;
        } else if peek(&iter, &mut index, "(") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::GoalHoOpen);
            continue;
        } else if peek(&iter, &mut index, ")") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::GoalHoClose);
            continue;
        } else if peek(&iter, &mut index, "{") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::MiddleGoalHoOpen);
            continue;
        } else if peek(&iter, &mut index, "}") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::MiddleGoalHoClose);
            continue;
        } else if peek(&iter, &mut index, ":") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::Column);
        } else if peek(&iter, &mut index, ",") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::Comma);
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
        } else {
            return result;
        }
    }
}
fn flush_keyword(buffer: &mut String, result: &mut Vec<Token>) {
    if !buffer.is_empty() {
        match buffer.as_str() {
            "함수마당" => {
                result.push(Token::Function);
            }
            "장군님께뜻올려드리기" => {
                result.push(Token::Return);
            }
            "만약장군님뜻대로라면" => {
                result.push(Token::If);
            }
            "장군님이불만족하시면다시감사하기" => result.push(Token::ElIf),
            "장군님판단에따라" => {
                result.push(Token::Else);
            }
            "충성반복" => {
                result.push(Token::Loop);
            }
            "장군님지시가이어지는동안" => {
                result.push(Token::While);
            }
            "장군님지시에따라" => {
                result.push(Token::For);
            }
            "충성스럽게하나씩" => {
                result.push(Token::In);
            }
            "위대하신장군님의명으로멈춤" => {
                result.push(Token::Break);
            }
            "충성하며계속" => {
                result.push(Token::Continue);
            }
            "장군님은혜로사용" => {
                result.push(Token::Use);
            }
            "장군님이열거하신위대한종류" => {
                result.push(Token::Enum);
            }
            "장군님식구조" => {
                result.push(Token::Struct);
            }
            "위대하신장군님뜻대로기능붙히기" => {
                result.push(Token::Impl);
            }
            "장군님께서이름을하사하시다" => {
                result.push(Token::Let);
            }
            "장군님께서정하신고정값" => {
                result.push(Token::Const);
            }
            "장군님이강조하신주체적인값" => {
                result.push(Token::Static);
            }
            "장군님의기계말" => {
                result.push(Token::Asm);
            }
            "장군님의선택에따른경우" => {
                result.push(Token::Match);
            }

            _ => {
                result.push(Token::KeyWord(std::mem::take(buffer)));
            }
        }
        buffer.clear();
    }
}
#[derive(Debug, PartialEq)]
enum Token {
    Function,
    TriangleOpen,
    TriangleClose,
    KeyWord(String),
    ReturnType,
    GoalHoOpen,
    GoalHoClose,
    MiddleGoalHoOpen,
    MiddleGoalHoClose,
    If,
    Else,
    ElIf,
    Loop,
    While,
    For,
    In,
    Break,
    Continue,
    Use,
    Enum,
    Struct,
    Impl,
    Let,
    Bstr(Vec<u8>),
    Static,
    Const,
    Asm,
    Match,
    Return,
    SemiColumn,
    Column,
    Str(String),
    Ignore(char),
    Comma,
}
fn peek(array: &Vec<char>, index: &mut usize, str: &str) -> bool {
    let index_failback = index.clone();
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
