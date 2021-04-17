mod tests {
    use crate::lexer::tokenize::tokenize;
    use crate::shared::token::{KeywordType, ContainerType, OperatorType};

    #[test]
    fn simple() {
        let result = tokenize(String::from("int 132211{ 32.85dd } >! = >="));

        assert_eq!(result.get(0).unwrap().clone().keyword.unwrap(), KeywordType::KwInt);
        assert_eq!(result.get(1).unwrap().clone().number.unwrap(), String::from("132211"));
        assert_eq!(result.get(3).unwrap().clone().number.unwrap(), String::from("32.85"));
    }

    #[test]
    fn code_sample() {
        let result = tokenize(String::from("export func main() { std::io::println(\"123456\"); return 0; }"));

        assert_eq!(result.get(0).unwrap().clone().keyword.unwrap(), KeywordType::KwExport);
        assert_eq!(result.get(1).unwrap().clone().keyword.unwrap(), KeywordType::KwFunc);
        assert_eq!(result.get(2).unwrap().clone().identifier.unwrap(), String::from("main"));
        assert_eq!(result.get(3).unwrap().clone().container.unwrap(), ContainerType::Bracket);
        assert_eq!(result.get(4).unwrap().clone().container.unwrap(), ContainerType::AntiBracket);
        assert_eq!(result.get(5).unwrap().clone().container.unwrap(), ContainerType::Brace);
        assert_eq!(result.get(6).unwrap().clone().identifier.unwrap(), String::from("std"));
        assert_eq!(result.get(7).unwrap().clone().operator.unwrap().operator_type, OperatorType::Scope);
        assert_eq!(result.get(8).unwrap().clone().identifier.unwrap(), String::from("io"));
        assert_eq!(result.get(12).unwrap().clone().string.unwrap(), String::from("123456"));
        assert_eq!(result.last().unwrap().clone().container.unwrap(), ContainerType::AntiBrace);
    }
}