mod common;

use std::{
    fs,
    path::{Path, PathBuf},
};

use common::run_test_folder;

use salite_checker::{analyzer::Analyzer, binder::Binder};
use salite_common::{Config, ConfigInfo};

use salite_parser::parse_file;
use salite_tokenizer::tokenize;

#[test]
fn test_pass_cases() {
    run_test_folder("./tests/cases/pass", &|path| {
        dbg!(path);
        let source = fs::read_to_string(path).expect("couldn't read the script file");
        let tokens = tokenize(&source).expect("failed to tokenize");
        let tokens = salite_ast::filter_non_trivia_tokens(tokens);
        let file = parse_file(&tokens).expect("failed to parse");

        let (binder, block) = Binder::new(&file);
        let config = Config::new(ConfigInfo::default(), PathBuf::from("."));
        let result = Analyzer::analyze(&binder, &config, &block);
        match result {
            Ok(_) => {}
            Err(error) => panic!("Failed: {}", error),
        }
    });
}

#[test]
fn test_fail_cases() {
    run_test_folder("./tests/cases/fail", &|path| {
        dbg!(path);
        let source = fs::read_to_string(path).expect("couldn't read the script file");
        let tokens = tokenize(&source).expect("failed to tokenize");
        let tokens = salite_ast::filter_non_trivia_tokens(tokens);
        let file = parse_file(&tokens).expect("failed to parse");

        let (binder, block) = Binder::new(&file);
        let config = Config::new(ConfigInfo::default(), PathBuf::from("."));
        let result = Analyzer::analyze(&binder, &config, &block);
        match result {
            Ok(_) => panic!("Expected fail"),
            Err(error) => {
                let output_path = Path::new(path).with_extension("error");
                fs::write(output_path, error.to_string()).expect("failed to write");
            }
        }
    });
}
