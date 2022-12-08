use std::path::Path;

use crate::common::test_example_file;

#[test]
fn lambda() {
    test_example_file(Path::new("examples/pass/lambda.uplp"))
}

#[test]
fn lambda_arg_let() {
    test_example_file(Path::new("examples/pass/lambda_arg_let.uplp"))
}

#[test]
fn lambda_with_let() {
    test_example_file(Path::new("examples/pass/lambda_with_let.uplp"))
}

#[test]
fn multi_arg_application() {
    test_example_file(Path::new("examples/pass/multi_arg_application.uplp"))
}

#[test]
fn fn_app_precedence() {
    test_example_file(Path::new("examples/pass/fn_app_precedence.uplp"))
}