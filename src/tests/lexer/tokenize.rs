use crate::lexer::tokenize::tokenize;
use crate::shared::token::container::ContainerType;
use crate::shared::token::keyword::KeywordType;
use crate::shared::token::operator::Operator;

#[test]
fn simple() {
    let result = tokenize("number 132211{ 32.85 dd } >! = >= ,", false);

    assert_eq!(
        result.get(0).unwrap().get_keyword().unwrap(),
        KeywordType::KwNumber
    );
    assert_eq!(
        result.get(2).unwrap().get_number().unwrap(),
        String::from("132211")
    );
    assert_eq!(
        result.get(5).unwrap().get_number().unwrap(),
        String::from("32.85")
    );
    assert_eq!(
        result
            .last()
            .unwrap()
            .get_operator()
            .unwrap(),
        Operator::Comma
    );
}

#[test]
fn code_sample() {
    let result = tokenize("export func main() { std::io::println(\"123456\"); return 0; }", false);

    assert_eq!(
        result.get(0).unwrap().get_keyword().unwrap(),
        KeywordType::KwExport
    );
    assert_eq!(
        result.get(2).unwrap().get_keyword().unwrap(),
        KeywordType::KwFunc
    );
    assert_eq!(
        result.get(4).unwrap().get_identifier().unwrap(),
        String::from("main")
    );
    assert_eq!(
        result.get(5).unwrap().get_container().unwrap(),
        ContainerType::Bracket
    );
    assert_eq!(
        result.get(6).unwrap().get_container().unwrap(),
        ContainerType::AntiBracket
    );
    assert_eq!(
        result.get(8).unwrap().get_container().unwrap(),
        ContainerType::Brace
    );
    assert_eq!(
        result.get(10).unwrap().get_identifier().unwrap(),
        String::from("std")
    );
    assert_eq!(
        result
            .get(11)
            .unwrap()
            .get_operator()
            .unwrap(),
        Operator::Scope
    );
    assert_eq!(
        result.get(12).unwrap().get_identifier().unwrap(),
        String::from("io")
    );
    assert_eq!(
        result.get(16).unwrap().get_string().unwrap(),
        String::from("123456")
    );
    assert_eq!(
        result.last().unwrap().get_container().unwrap(),
        ContainerType::AntiBrace
    );
}

#[test]
fn comment_test() {
    let result = tokenize("// comments here", false);

    assert_eq!(result.len(), 1);
}
