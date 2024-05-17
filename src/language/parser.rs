use std::{any::Any, rc::Rc};

use crate::language::types::Operator;

use super::types::Unit;

#[derive(Clone)]
pub struct Parser {
    string: String,
}

impl Parser {
    pub fn new(string: String) -> Parser {
        Parser {
            string: string,
        }
    }

    fn parse_part(&self, begin: usize, end: usize) -> Result<Rc<Unit>, String> {
        let mut pair: (Option<(usize, usize)>, Option<(usize, usize)>) = (
            None,
            None,
        );
        let chars = self.string.chars().collect::<Vec<char>>();
        let mut deep = 0;
        let mut maximal_operator: Option<(usize, char)> = None;
        let operators = "→∧∨¬".chars().collect::<Vec<char>>();
        for i in begin..end  {
            if chars[i] == '(' {
                deep += 1;
                continue;
            } else if chars[i] == ')' {
                deep -= 1;
                continue;
            }
            if deep == 0 && operators.contains(&chars[i]) {
                match maximal_operator {
                    Some((_, char)) => {
                        match chars[i] {
                            '→' => {
                                maximal_operator = Some((i, chars[i]));
                            },
                            '∧' => {
                                if char == '∨' || char == '¬' {
                                    maximal_operator = Some((i, chars[i]));
                                }
                            },
                            '∨' => {
                                if char == '¬' {
                                    maximal_operator = Some((i, chars[i]));
                                }
                            },
                            _ => {},
                        }
                    },
                    None => {
                        maximal_operator = Some((i, chars[i]));
                    },
                }
            }
        };
        match maximal_operator {
            Some(maximal_operator) => {
                match maximal_operator.1 {
                    '→' => {
                        Ok(Unit::duo(Operator::Implies, (
                            self.parse_part(begin, maximal_operator.0).unwrap(),
                            self.parse_part(maximal_operator.0 + 1, end).unwrap()
                        )))
                    },
                    '∨' => {
                        Ok(Unit::duo(Operator::Or, (
                            self.parse_part(begin, maximal_operator.0).unwrap(),
                            self.parse_part(maximal_operator.0 + 1, end).unwrap()
                        )))
                    },
                    '∧' => {
                        Ok(Unit::duo(Operator::And, (
                            self.parse_part(begin, maximal_operator.0).unwrap(),
                            self.parse_part(maximal_operator.0 + 1, end).unwrap()
                        )))
                    },
                    '¬' => {
                        match self.parse_part(maximal_operator.0 + 1, end) {
                            Ok(unit) => {
                                Ok(Unit::uno(Operator::Not, unit))
                            },
                            Err(err) => Err(err),
                        }
                    },
                    _ => {
                        Ok(Unit::predicate(maximal_operator.1))
                    },
                }
            },
            None => {
                if end - begin == 1 {
                    Ok(Unit::predicate(chars[begin]))
                } else {
                    self.parse_part(begin + 1, end - 1)
                }
            },
        }
    }

    pub fn parse(&self) -> Result<Rc<Unit>, String> {
        self.parse_part(0, self.string.chars().count())
    }
}
