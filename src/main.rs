use language::{analyzer::{analyse, Branch}, types::{Operator, Unit}};

mod language;

fn main() {
    let node = Unit::uno(
        Operator::Not,
        Unit::duo(
            Operator::Implies,
            (
                Unit::predicate('i'),
                Unit::duo(
                    Operator::Or,
                    (
                        Unit::predicate('j'),
                        Unit::predicate('i'),
                    )
                ),
            )
        )
    );
    let node = Unit::duo(
        Operator::Implies,
        (
            Unit::predicate('i'),
            Unit::duo(
                Operator::Or,
                (
                    Unit::predicate('j'),
                    Unit::predicate('i'),
                )
            ),
        )
    );

    analyse(Branch::from(node), 0);
}
