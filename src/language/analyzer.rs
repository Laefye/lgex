use std::{cmp::{self, Ordering}, collections::HashMap, rc::Rc};

use super::types::{Operator, Predicate, Unit};

#[derive(Clone)]
struct PredicateState {
    predicate: Predicate,
    not: bool,
}

impl PredicateState {
    fn from_unit(value: &Rc<Unit>) -> Option<PredicateState> {
        match value.as_ref() {
            Unit::Predicate(predicate) => Some(
                PredicateState::from_predicate(predicate)
            ),
            _ => None,
        }
    }
    fn from_predicate(predicate: &Predicate) -> PredicateState {
        PredicateState {
            not: false,
            predicate: predicate.clone(),
        }
    }
    fn not(mut self) -> PredicateState {
        self.not = true;
        self
    }
}

#[derive(Clone)]
struct Memory {
    stack: Vec<Rc<Unit>>,
    predicates: Vec<PredicateState>,
}

#[derive(Clone)]
pub struct Branch {
    memory: Memory,
}

#[derive(Clone)]
pub enum TreeResult {
    Continue(),
    Born(Branch),
}

#[derive(Clone)]
pub enum Model {
    NotExists,
    Exist(HashMap<char, bool>)
}

impl Branch {
    fn remove_unit(&mut self, unit: &Rc<Unit>) {
        match self.memory.stack.iter().position(|x| Rc::ptr_eq(x, unit)) {
            Some(i) => {
                self.memory.stack.remove(i);
            },
            None => {},
        }
    }

    pub fn has(&self) -> bool {
        self.memory.stack.len() > 0
    }

    pub fn step(&mut self) -> TreeResult {
        let mut amount: Vec<Rc<Unit>> = self.memory.stack.clone();
        amount.sort_by(|x, y| y.cmp(x).reverse());
        let unit = amount.first();
        let mut new_branch = Option::<Branch>::None;
        match unit {
            Some(unit) => {
                match unit.as_ref() {
                    Unit::Predicate(predicate) => {
                        self.memory.predicates.push(PredicateState::from_predicate(predicate));
                        self.remove_unit(unit);
                    },
                    Unit::Uno(uno) => {
                        match uno.operator() {
                            Operator::Not => {
                                match uno.unit().clone().as_ref() {
                                    Unit::Predicate(predicate) => {
                                        self.memory.predicates.push(PredicateState::from_predicate(predicate).not());
                                        self.remove_unit(unit);
                                    },
                                    Unit::Uno(uno) => {
                                        match uno.operator() {
                                            Operator::Not => {
                                                self.memory.stack.push(uno.unit())
                                            },
                                            _ => {}
                                        }
                                        self.remove_unit(unit);
                                    },
                                    Unit::Duo(duo) => {
                                        match duo.operator() {
                                            Operator::And => {
                                                let units = duo.units();
                                                let mut unwrapped_new_brance = self.clone();
                                                self.memory.stack.push(Unit::uno(Operator::Not, units.0));
                                                unwrapped_new_brance.memory.stack.push(Unit::uno(Operator::Not, units.1));

                                                unwrapped_new_brance.remove_unit(unit);
                                                self.remove_unit(unit);
                                                new_branch = Some(unwrapped_new_brance);
                                            },
                                            Operator::Or => {
                                                let units = duo.units();
                                                self.memory.stack.push(Unit::uno(Operator::Not, units.0));
                                                self.memory.stack.push(Unit::uno(Operator::Not, units.1));
                                                self.remove_unit(unit);
                                            },
                                            Operator::Implies => {
                                                let units = duo.units();
                                                self.memory.stack.push(units.0);
                                                self.memory.stack.push(Unit::uno(Operator::Not, units.1));
                                                self.remove_unit(unit);
                                            },
                                            _ => {}
                                        }
                                    },
                                }
                            },
                            _ => {},
                        }
                    },
                    Unit::Duo(duo) => {
                        match duo.operator() {
                            Operator::And => {
                                let units = duo.units();
                                self.memory.stack.push(units.0);
                                self.memory.stack.push(units.1);
                                self.remove_unit(unit);
                            },
                            Operator::Or => {
                                let units = duo.units();
                                let mut unwrapped_new_brance = self.clone();
                                self.memory.stack.push(units.0);
                                unwrapped_new_brance.memory.stack.push(units.1);
                                unwrapped_new_brance.remove_unit(unit);
                                self.remove_unit(unit);
                                new_branch = Some(unwrapped_new_brance);
                            },
                            Operator::Implies => {
                                let units = duo.units();
                                let mut unwrapped_new_brance = self.clone();
                                self.memory.stack.push(Unit::uno(Operator::Not, units.0));
                                unwrapped_new_brance.memory.stack.push(units.1);
                                unwrapped_new_brance.remove_unit(unit);
                                self.remove_unit(unit);
                                new_branch = Some(unwrapped_new_brance);
                            },
                            _ => {}
                        }
                    },
                }
            },
            _ => {},
        };
        match new_branch {
            Some(branch) => TreeResult::Born(branch),
            None => TreeResult::Continue(),
        }
    }

    fn model(&self) -> Model {
        let mut map = HashMap::<char, bool>::new();
        for predicate in self.memory.predicates.clone() {
            if map.contains_key(&predicate.predicate.char()) && map[&predicate.predicate.char()] != predicate.not {
                return Model::NotExists;
            }
            map.insert(predicate.predicate.char(), predicate.not);
        }
        Model::Exist(map)
    } 
}

impl ToString for Branch {
    fn to_string(&self) -> String {
        let stack = self.memory.stack.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
        let predicates = self.memory.predicates.iter().map(|x: &PredicateState| {
            match x.not {
                true => {
                    let mut str = "!".to_string();
                    str.push(x.predicate.char());
                    str
                },
                false => x.predicate.char().to_string(),
            }
        }).collect::<Vec<String>>().join(" ");
        format!("({}) ({})", stack, predicates)
    }
}

impl From<Rc<Unit>> for Branch {
    fn from(value: Rc<Unit>) -> Self {
        Branch {
            memory: Memory {
                stack: vec![value],
                predicates: Vec::new(),
            }
        }
    }
}

pub fn analyse(mut branch: Branch, i: usize) {
    println!("{}{}", "   ".repeat(i), branch.to_string());
    while branch.has() {
        let result = branch.step();
        match result {
            TreeResult::Continue() => {},
            TreeResult::Born(branch) => {
                analyse(branch, i + 1);
            },
        };
        println!("{}{}", "   ".repeat(i), branch.to_string());
    }
    let vars = branch.model();
    match vars {
        Model::NotExists => {},
        Model::Exist(table) => {
            let result = table.iter().map(|(x, y)| format!("{}={}", x, !y)).collect::<Vec<String>>().join(" ");
            println!("{}{}", "   ".repeat(i), result);
        },
    }
}
