use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::error::general_error::GeneralError;

pub fn link_statement_builder(
    tokens: &Vec<DecoratedToken>,
) -> Result<(String, usize), GeneralError<String>> {
    if tokens.len() >= 3 {
        let next_semicolon_pos = find_next_semicolon(tokens.clone());

        if next_semicolon_pos.unwrap_or(0) == 2 {
            if tokens[0].content.get_decorated_keyword().is_some()
                && tokens[1].content.is_valid_identifier()
            {
                return Ok((tokens[1].content.get_data().unwrap().get_identifier().unwrap().clone(), 2));
            }
        }
    }

    return Err(GeneralError {
        code: "-1".to_string(),
        description: None,
    });
}
