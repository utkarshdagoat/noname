use std::path::Path;

use crate::{ast::Compiler, lexer::Token, parser::AST};

fn test_file(file_name: &str, expected_gates: usize, expected_asm: &str) {
    let version = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(version).join("data").join(file_name);

    let code = std::fs::read_to_string(path).unwrap();

    let tokens = Token::parse(&code).unwrap();
    let ast = AST::parse(tokens).unwrap();
    let (circuit, compiler) = Compiler::analyze_and_compile(ast, &code, false).unwrap();

    assert_eq!(compiler.num_gates(), expected_gates);

    assert_eq!(circuit, expected_asm);
}

#[test]
fn test_arithmetic() {
    let asm = "@ noname.0.1.0

DoubleGeneric<1>
DoubleGeneric<1,1,-1>
DoubleGeneric<1,0,0,0,-2>
DoubleGeneric<1,-1>
(0,0) -> (1,1)
(1,2) -> (3,0)
(2,0) -> (3,1)
";
    test_file("arithmetic.no", 4, asm);
}

#[test]
fn test_public_output() {
    let asm = "@ noname.0.1.0

DoubleGeneric<1>
DoubleGeneric<1>
DoubleGeneric<1,1,-1>
DoubleGeneric<1,0,0,0,-2>
DoubleGeneric<1,-1>
DoubleGeneric<1,0,0,0,-6>
DoubleGeneric<1,1,-1>
(0,0) -> (2,1)
(2,2) -> (4,0) -> (6,0)
(3,0) -> (4,1)
(5,0) -> (6,1)
";

    test_file("public_output.no", 7, asm);
}