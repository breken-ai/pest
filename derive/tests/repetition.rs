// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
extern crate pest;
extern crate pest_derive;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
item        = { ASCII_ALPHA }
list        = { (item ~ ",")* ~ item }
list_atomic = @{ ("x" ~ "y")* ~ "x" }
list_normal = { ("x" ~ "y")* ~ "x" }
file        = { SOI ~ list ~ ","? ~ EOI }
file_inline = { SOI ~ (item ~ ",")* ~ item ~ ","? ~ EOI }
"#]
struct RepetitionParser;

#[test]
fn greedy_repetition_does_not_give_back_the_last_item() {
    // `(item ~ ",")*` consumes "a,b," so the trailing `item` has nothing left
    // to match; repetitions never backtrack.
    assert!(RepetitionParser::parse(Rule::list, "a,b,").is_err());
    assert_eq!(
        RepetitionParser::parse(Rule::list, "a,b").unwrap().as_str(),
        "a,b"
    );
}

#[test]
fn atomic_and_non_atomic_repetition_agree() {
    assert!(RepetitionParser::parse(Rule::list_atomic, "xy").is_err());
    assert!(RepetitionParser::parse(Rule::list_normal, "xy").is_err());
    assert!(RepetitionParser::parse(Rule::list_atomic, "xyx").is_ok());
    assert!(RepetitionParser::parse(Rule::list_normal, "xyx").is_ok());
}

#[test]
fn extracting_a_list_rule_does_not_change_the_language() {
    for input in ["a", "a,b", "a,b,"] {
        assert_eq!(
            RepetitionParser::parse(Rule::file, input).is_ok(),
            RepetitionParser::parse(Rule::file_inline, input).is_ok(),
            "{input:?}"
        );
    }
}
