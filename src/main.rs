use language::{analyzer::{analyse, Branch}, types::{Operator, Unit}};

mod language;

fn main() {
    // ((p→q)→r)→(¬r→(p∧¬q))
    let node = Unit::uno(
        Operator::Not,
        Unit::duo(
            Operator::Implies,
            (
                Unit::duo(
                    Operator::Implies,
                    (
                        Unit::duo(
                            Operator::Implies,
                            (
                                Unit::predicate('p'),
                                Unit::predicate('q'),
                            )
                        ),
                        Unit::predicate('r'),
                    )
                ),
                Unit::duo(
                    Operator::Implies,
                    (
                        Unit::uno(Operator::Not, Unit::predicate('r')),
                        Unit::duo(
                            Operator::And,
                            (
                                Unit::predicate('p'),
                                Unit::uno(Operator::Not, Unit::predicate('q')),
                            )
                        ),
                    )
                ),
            )
        )
    );
    analyse(Branch::from(node), 0);
}
