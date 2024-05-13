use std::rc::Rc;

#[derive(Clone)]
pub enum Operator {
    Not,
    And,
    Or,
    Implies
}

impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::Not => "~",
            Operator::And => "*",
            Operator::Or => "+",
            Operator::Implies => ">",
        }.to_string()
    }
}

#[derive(Clone)]
pub enum Unit {
    Predicate(Predicate),
    Uno(Uno),
    Duo(Duo),
}

#[derive(Clone)]
pub struct Predicate {
    char: char,
}

impl Predicate {
    pub fn char(&self) -> char {
        self.char
    }
}

#[derive(Clone)]
pub struct Uno {
    operator: Operator,
    unit: Rc<Unit>,
}

impl Uno {
    pub fn operator(&self) -> Operator {
        self.operator.clone()
    }

    pub fn unit(&self) -> Rc<Unit> {
        self.unit.clone()
    }
}

#[derive(Clone)]
pub struct Duo {
    operator: Operator,
    units: (Rc<Unit>, Rc<Unit>),
}

impl Duo  {
    pub fn operator(&self) -> Operator {
        self.operator.clone()
    }

    pub fn units(&self) -> (Rc<Unit>, Rc<Unit>) {
        (self.units.0.clone(), self.units.1.clone())
    }
}

impl Unit {
    pub fn uno(operator: Operator, unit: Rc<Unit>) -> Rc<Unit> {
        Rc::new(Unit::Uno(Uno { operator: operator, unit: unit }))
    }

    pub fn duo(operator: Operator, units: (Rc<Unit>, Rc<Unit>)) -> Rc<Unit> {
        Rc::new(Unit::Duo(Duo { operator: operator, units: units }))
    }

    pub fn predicate(char: char) -> Rc<Unit> {
        Rc::new(Unit::Predicate(Predicate { char: char }))
    }

    fn cost(&self) -> i32 {
        match self {
            Unit::Predicate(_) => 0,
            Unit::Uno(_) => 0,
            Unit::Duo(duo) => {
                match duo.operator {
                    Operator::Not => 0,
                    Operator::And => 1,
                    Operator::Or => 2,
                    Operator::Implies => 0,
                }
            },
        }
    }

    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost().cmp(&other.cost())
    }
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::Predicate(predicate) => format!("{}", predicate.char),
            Unit::Uno(uno) => format!("{}({})", uno.operator.to_string(), uno.unit.to_string()),
            Unit::Duo(duo) => format!("({} {} {})", duo.units.0.to_string(), duo.operator.to_string(), duo.units.1.to_string()),
        }
    }
}
