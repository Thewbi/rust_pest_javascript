use std::fs;

use pest::{Parser, iterators::{Pair, Pairs}};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "javascript.pest"]
pub struct CSVParser;

fn main() {

    println!("Hello, world!");

    //let filename = "examples/snippets/single_line_comments.js";
    //let filename = "examples/snippets/assignment.js";
    let filename = "examples/snippets/assignment_2.js";
    //let filename = "examples/snippets/function.js";
    //let filename = "examples/snippets/function_2.js";
    //let filename = "examples/snippets/let.js";
    //let filename = "examples/snippets/add_expression.js";
    //let filename = "examples/snippets/let_string.js";
    //let filename = "examples/snippets/let_multiple.js";
    //let filename = "examples/snippets/nested_expression.js";
    //let filename = "examples/snippets/string_concat.js";

    let src = fs::read_to_string(&filename).expect("Failed to read file");

    //let successful_parse = CSVParser::parse(Rule::compilation_unit, "function test{}");
    let successful_parse = CSVParser::parse(Rule::compilation_unit, &src);
    //println!("{:?}", successful_parse);

    // There should be a single root node in the parsed tree
    let res: Result<Pairs<'_, Rule>, pest::error::Error<Rule>> = successful_parse;

    let mut pairs = res.expect("Parsing failed");
    //println!("{}", pairs);
    //println!("{:?}", pairs);

    recurse_pairs(&mut pairs, 0);

    // // Because ident_list is silent, the iterator will contain idents
    // for pair in pairs {

    //     // A pair is a combination of the rule which matched and a span of input
    //     println!("Rule:    {:?}", pair.as_rule());
    //     println!("Span:    {:?}", pair.as_span());
    //     println!("Text:    {}", pair.as_str());

    //     // // A pair can be converted to an iterator of the tokens which make it up:
    //     // for inner_pair in pair.into_inner() {
    //     //     match inner_pair.as_rule() {
    //     //         Rule::compilation_unit => println!("compilation_unit: {}", inner_pair.as_str()),
    //     //         Rule::function_declaration => println!("function_declaration: {}", inner_pair.as_str()),
    //     //         _ => unreachable!()
    //     //     };
    //     // }
    // }

    // Consume the `Node` recursively into the final value
    //CSVParser::file(input)

    // let unsuccessful_parse = CSVParser::parse(Rule::field, "this is not a number");
    // println!("{:?}", unsuccessful_parse);
}

fn recurse_pairs(pairs: &mut Pairs<'_, Rule>, indent: usize) {

    // Because ident_list is silent, the iterator will contain idents
    for mut pair in pairs {

        recurse_pair(&mut pair, indent);

        // // A pair can be converted to an iterator of the tokens which make it up:
        // for inner_pair in pair.into_inner() {
        //     match inner_pair.as_rule() {
        //         Rule::compilation_unit => println!("compilation_unit: {}", inner_pair.as_str()),
        //         Rule::function_declaration => println!("function_declaration: {}", inner_pair.as_str()),
        //         _ => unreachable!()
        //     };
        // }
    }
}

fn recurse_pair(pair: &mut Pair<'_, Rule>, indent: usize) {

    let indent_string = "  ".repeat(indent);

    match pair.as_rule() {
        Rule::function_declaration => println!("function_declaration: {}", pair.as_str()),
         _ => {}
    }

    // A pair is a combination of the rule which matched and a span of input
    println!("{}Rule:    {:?}", indent_string, pair.as_rule());
    //println!("Span:    {:?}", pair.as_span());
    //println!("Text:    {}", pair.as_str());

    for mut inner_pair in pair.clone().into_inner() {
        recurse_pair(&mut inner_pair, indent+1);
    }

}
