use crate::package_generator::utils::find_function;
use crate::shared::ast::action::CallAction;
use crate::shared::ast::blocks::function::Function;

// Check CallAction
pub fn check_function_existence(function_table: &Vec<Function>, action: &CallAction) -> bool {
    return find_function(action.function_name.as_str(), function_table).is_some();
}
