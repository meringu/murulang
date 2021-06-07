mod argument_error;
mod function_case_missing_error;
mod function_case_unreachable_error;
mod function_not_found_error;
mod no_function_matches_error;
mod not_implemented_error;
mod operator_argument_error;
mod standard_error;
mod type_mismatch_error;
mod untyped_function_error;

pub use crate::err::argument_error::ArgumentError;
pub use crate::err::function_case_missing_error::FunctionCaseMissingError;
pub use crate::err::function_case_unreachable_error::FunctionCaseUnreachableError;
pub use crate::err::function_not_found_error::FunctionNotFoundError;
pub use crate::err::no_function_matches_error::NoFunctionMatchesError;
pub use crate::err::not_implemented_error::NotImplementedError;
pub use crate::err::operator_argument_error::OperatorArgumentError;
pub use crate::err::standard_error::StandardError;
pub use crate::err::type_mismatch_error::TypeMismatchError;
pub use crate::err::untyped_function_error::UntypedFunctionError;
