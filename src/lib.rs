use pest_derive::Parser;
use lalrpop_util::lalrpop_mod;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct PestParser;

lalrpop_mod!(grammar);