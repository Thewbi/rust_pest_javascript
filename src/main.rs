use std::fs;

use pest::{Parser, iterators::{Pair, Pairs}};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "javascript.pest"]
pub struct CSVParser;

fn main() {

    println!("Hello, world!");

    //let filename = "examples/snippets/scratchpad.js";

    //let filename = "examples/snippets/single_line_comments.js";

    // let filename = "examples/snippets/assignment.js";
    // let src = fs::read_to_string(&filename).expect("Failed to read file");
    // let successful_parse = CSVParser::parse(Rule::compilation_unit, &src);

    //let filename = "examples/snippets/assignment_expression.js";
    //let filename = "examples/snippets/relational_expression.js";
    //let filename = "examples/snippets/assignment_2.js";
    //let filename = "examples/snippets/function.js";

    // let filename = "examples/snippets/function_2.js";
    // let src = fs::read_to_string(&filename).expect("Failed to read file");
    // let successful_parse = CSVParser::parse(Rule::compilation_unit, &src);

    // let filename = "examples/snippets/if_statement.js";
    // let src = fs::read_to_string(&filename).expect("Failed to read file");
    // let successful_parse = CSVParser::parse(Rule::compilation_unit, &src);

    // let filename = "examples/snippets/if_else_statement.js";
    // let src = fs::read_to_string(&filename).expect("Failed to read file");
    // let successful_parse = CSVParser::parse(Rule::compilation_unit, &src);

    // let filename = "examples/snippets/else_if_statement.js";
    // let src = fs::read_to_string(&filename).expect("Failed to read file");
    // let successful_parse = CSVParser::parse(Rule::compilation_unit, &src);

    // let filename = "examples/snippets/for_loop.js";
    // let src = fs::read_to_string(&filename).expect("Failed to read file");
    // let successful_parse = CSVParser::parse(Rule::compilation_unit, &src);

    // let filename = "examples/snippets/while_loop.js";
    // let src = fs::read_to_string(&filename).expect("Failed to read file");
    // let successful_parse = CSVParser::parse(Rule::compilation_unit, &src);

    let filename = "examples/snippets/switch_statement.js";
    let src = fs::read_to_string(&filename).expect("Failed to read file");
    let successful_parse = CSVParser::parse(Rule::compilation_unit, &src);

    // let filename = "examples/snippets/string_literal.js";
    // let src = fs::read_to_string(&filename).expect("Failed to read file");
    // let successful_parse = CSVParser::parse(Rule::string_literal, &src);

    // let filename = "examples/snippets/less_than_expression.js";
    // let src = fs::read_to_string(&filename).expect("Failed to read file");
    // let successful_parse = CSVParser::parse(Rule::expression, &src);

    // let filename = "examples/snippets/update_expression.js";
    // let src = fs::read_to_string(&filename).expect("Failed to read file");
    // let successful_parse = CSVParser::parse(Rule::expression, &src);

    // let filename = "examples/snippets/lexical_declaration.js";
    // let src = fs::read_to_string(&filename).expect("Failed to read file");
    // let successful_parse = CSVParser::parse(Rule::lexical_declaration, &src);

    //let filename = "examples/snippets/let.js";
    //let filename = "examples/snippets/add_expression.js";
    //let filename = "examples/snippets/let_string.js";
    //let filename = "examples/snippets/let_multiple.js";
    //let filename = "examples/snippets/nested_expression.js";
    //let filename = "examples/snippets/string_concat.js";

    //let src = fs::read_to_string(&filename).expect("Failed to read file");

    //let successful_parse = CSVParser::parse(Rule::compilation_unit, "function test{}");
    //let successful_parse = CSVParser::parse(Rule::compilation_unit, &src);

    //let successful_parse = CSVParser::parse(Rule::if_statement, &src);



    //let successful_parse = CSVParser::parse(Rule::assignment_expression, &src);

    //let successful_parse = CSVParser::parse(Rule::relational_expression, &src);

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

    // because ident_list is silent, the iterator will contain idents
    for mut pair in pairs {

        recurse_pair(&mut pair, indent);
    }
}

fn recurse_pair(pair: &mut Pair<'_, Rule>, indent: usize) {

    let indent_string = "  ".repeat(indent);

    //
    // before the recursion
    //

    // a pair is a combination of the rule which matched and a span of input
    // print!("{}Rule: {:?}", indent_string, pair.as_rule());
    // //println!("Span:    {:?}", pair.as_span());
    // println!(", Text: '{}'", pair.as_str().trim());

    for mut inner_pair in pair.clone().into_inner() {
        recurse_pair(&mut inner_pair, indent+1);
    }

    //
    // after the recursion
    //

    let pair_as_rule = pair.as_rule();

    match pair_as_rule {

        Rule::switch_statement => {
            println!("switch_statement: {}", pair.as_str());
        },

        Rule::string_literal => {
            println!("string_literal: {}", pair.as_str());
        },

        Rule::function_declaration => {
            println!("function_declaration: {}", pair.as_str());
        },

        Rule::lexical_declaration => {
            println!("lexical_declaration: {}", pair.as_str());
        }

        Rule::expression => {
            println!("expression: {}", pair.as_str());
        }

        Rule::lexical_binding => {
            println!("lexical_binding: {}", pair.as_str());
            // for mut inner_pair in pair.clone().into_inner() {
            //     match pair.as_rule() {
            //         _ => {}
            //     }
            // }
            for child in pair.clone().into_inner() {
                println!("Child: {:?}", child.as_rule());
                println!("Text: {:?}", child.as_str());
            }

            // identifier
            let child_0 = pair.clone().into_inner().nth(0);
            println!("child_0: {:?}", child_0.unwrap().as_str());

            // assigned expression
            let child_1 = pair.clone().into_inner().nth(1);
            if child_1.is_some() {
                println!("child_1: {:?}", child_1.unwrap().as_str());
            }

            // TODO: after ascending from the recursion, take the
            // latest node as assigned expression that was created by the recursion
        },

         _ => {}
    }

}
