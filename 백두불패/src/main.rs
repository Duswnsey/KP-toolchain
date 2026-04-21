/*
 * 백두불페:조선식 전자계산기말
 * copyright (c) 2026 련준됴
*/

use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input);
    let token_tree = lexer(input);
    let mut index = 0;
    let ast = parser(&token_tree, &mut index, true);
    println!("{:#?}", ast);
}
fn parser(token_tree: &Vec<Token>, index: &mut usize, top: bool) -> Vec<Element> {
    let mut result = Vec::new();
    while token_tree.get(*index) != None {
        if !top && token_tree.get(*index) == Some(&Token::MiddleGoalHoClose) {
            *index += 1;
            return result;
        }
        match get_statement(token_tree, index) {
            Some(element) => {
                result.push(element);
            }
            _ => {
                let value = get_expression(token_tree, index, 0, true);
                if token_tree.get(*index) == Some(&Token::SemiColumn) {
                    *index += 1;
                    result.push(Element::Thatsit(value));
                } else {
                    result.push(Element::Expose(value));
                }
            }
        }
    }
    result
}
fn get_statement(token_tree: &Vec<Token>, index: &mut usize) -> Option<Element> {
    let Some(token) = token_tree.get(*index) else {
        panic!()
    };
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
                let typee = get_pattern(&token_tree, index);
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
                Some(get_pattern(&token_tree, index))
            } else {
                None
            };
            let scope = if token_tree.get(*index) == Some(&Token::MiddleGoalHoOpen) {
                if let Expression::Scope(scope) = get_expression(token_tree, index, 0, false) {
                    Some(scope)
                } else {
                    panic!()
                }
            } else {
                None
            };
            return Some(Element::Function {
                name: name.to_string(),
                argument: arguments,
                return_type: return_type,
                code: scope,
            });
        }
        Token::Clang => {
            let mut elements = Vec::new();
            if token_tree.get(*index) != Some(&Token::MiddleGoalHoOpen) {
                panic!()
            }

            *index += 1;
            loop {
                if token_tree.get(*index) == Some(&Token::MiddleGoalHoClose) {
                    *index += 1;
                    break;
                }
                let item = get_statement(token_tree, index);
                if !matches!(item, Some(Element::Function{name:_,return_type:_,argument:_, code:_})) {
                    panic!();
                }
                if token_tree.get(*index) != Some(&Token::SemiColumn) {
                    panic!()
                }
                *index += 1;
                
                elements.push(item?);
                
            }
return Some(Element::Clang(elements));
        }
        _ => {
            *index -= 1;
            return None;
        }
    }
    None
}
fn get_pattern(tree: &[Token], index: &mut usize) -> Pattern {
    Pattern::NotImplementedPattern
}
fn get_expression(tree: &Vec<Token>, index: &mut usize, offset: usize, full: bool) -> Expression {
    let mut result;
    match tree.get(*index + offset) {
        Some(Token::KeyWord(name)) => {
            *index += 1;
            if tree.get(*index) == Some(&Token::GoalHoOpen) {
                let arguments: Vec<Expression> =
                    if let Expression::Tuple(tup) = get_expression(tree, index, 0, false) {
                        tup.clone()
                    } else {
                        panic!();
                    };
                result = Expression::Function(name.to_string(), arguments);
            } else {
                result = Expression::Other(name.to_string());
            }
        }
        Some(Token::GoalHoOpen) => {
            *index += 1;
            result = Expression::Tuple(Vec::new());
            if tree.get(*index) == Some(&Token::GoalHoClose) {
                *index += 1;
            } else {
                loop {
                    if let Expression::Tuple(ref mut tup) = result {
                        tup.push(get_expression(tree, index, 0, true));
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
        }
        Some(Token::MiddleGoalHoOpen) => {
            *index += 1;
            result = Expression::Scope(parser(tree, index, false));
        }
        Some(Token::Str(string)) => {
            *index += 1;
            result = Expression::Str(string.clone());
        }
        _ => {
            panic!()
        }
    }
    if full {
        loop {
            match tree.get(*index) {
                Some(&Token::Dot) => {
                    *index += 1;
                    result = Expression::Chain(
                        Box::new(result),
                        Box::new(get_expression(tree, index, 0, false)),
                    );
                }
                Some(&Token::Add) => {
                    *index += 1;
                    expression_calculate(&mut result, CalculationType::Add, index, tree);
                }
                Some(&Token::Multifly) => {
                    *index += 1;
                    expression_calculate(&mut result, CalculationType::Multiply, index, tree);
                }
                Some(&Token::Square) => {
                    *index += 1;
                    expression_calculate(&mut result, CalculationType::Square, index, tree);
                }
                Some(&Token::Equal) => {
                    *index += 1;
                    expression_calculate(&mut result, CalculationType::Equal, index, tree);
                }
                _ => break,
            }
        }
    }
    result
}
//사칙련산 관련 처리
fn expression_calculate(
    current: &mut Expression,
    calculation_type: CalculationType,
    index: &mut usize,
    tree: &Vec<Token>,
) {
    //앞 표현식이 련산 고려 대상이 아니라면
    if !matches!(current, Expression::Calculation(_, _, _)) {
        *current = Expression::Calculation(
            calculation_type,
            Box::new(current.clone()),
            Box::new(get_expression(tree, index, 0, false)),
        );
        return;
    }
    let cloned = current.clone();
    if let Expression::Calculation(current_calc, _, s) = current {
        if current_calc.get_priority() < calculation_type.get_priority() {
            expression_calculate(&mut *s, calculation_type.clone(), index, tree);
        } else {
            *current = Expression::Calculation(
                calculation_type.clone(),
                Box::new(cloned),
                Box::new(get_expression(tree, index, 0, false)),
            )
        }
    }
}
impl CalculationType {
    fn get_priority(&self) -> usize {
        match self {
            CalculationType::Equal => 0,
            CalculationType::Add => 1,
            CalculationType::Multiply => 2,
            CalculationType::Square => 3,
        }
    }
}
#[derive(Debug, Clone)]
enum CalculationType {
    Add,
    Multiply,
    Square,
    Equal,
}
#[derive(Debug, Clone)]
enum Pattern {
    NotImplementedPattern,
}
type Ast = Vec<Element>;
#[derive(Debug, Clone)]
enum Element {
    //statement
    Function {
        argument: Vec<(String, Pattern)>,
        name: String,
        code: Option<Ast>,
        return_type: Option<Pattern>,
    },
    Clang(Vec<Element>),
    Expose(Expression),
    Thatsit(Expression),
}
#[derive(Debug, Clone)]
enum Expression {
    Tuple(Vec<Expression>),
    Other(String),
    Function(String, Vec<Expression>),
    Str(String),
    Chain(Box<Expression>, Box<Expression>),
    // 사슬 함수로 변환할 예정
    // 사슬함수에서 연산자를 호출하는것도 사칙 련산이 적용되기 때문 */
    Calculation(CalculationType, Box<Expression>, Box<Expression>),
    Scope(Ast),
    If {
        r#if: (Box<Expression>, Ast),
        elif: Option<(Box<Expression>, Ast)>,
    },
}

fn lexer(raw: String) -> Vec<Token> {
    let iter: Vec<char> = raw.chars().collect();
    let mut index = 0;
    let mut result = Vec::new();
    //문자 핵심어를 임시적으로 수용함
    let mut keyword = String::new();
    //기호 핵심어를 처리함.
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
        } else if peek(&iter, &mut index, "+") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::Add);
        } else if peek(&iter, &mut index, "*") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::Multifly);
        } else if peek(&iter, &mut index, "^") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::Square);
        } else if peek(&iter, &mut index, "==") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::Equal);
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
        } else if peek(&iter, &mut index, ".") {
            flush_keyword(&mut keyword, &mut result);
            result.push(Token::Dot);
        } else if let Some(ch) = iter.get(index) {
            keyword.push(*ch);
            index += 1;
        } else {
            return result;
        }
    }
}
//문자열 핵심어를 처리
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
            "clang" => {
                //C언어 생 프로그람 호출소
                result.push(Token::Clang);
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
    Add,
    Asm,
    Bstr(Vec<u8>),
    Clang,
    Const,
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
    Static,
    Match,
    Return,
    SemiColumn,
    Column,
    Str(String),
    Ignore(char),
    Comma,
    Dot,
    Multifly,
    Square,
    Equal,
}
//문자열 엿보는 도우미 함수마당
fn peek(array: &[char], index: &mut usize, str: &str) -> bool {
    let index_failback = *index;
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
