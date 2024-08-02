use crate::cursor::CursorNavigation;

pub enum LoopThrough<'a, T> {
    /// Run until the end of the source
    All,

    /// Run while the condition is true
    While(&'a dyn Fn(&T) -> bool),

    /// Run while the current unit is equal to the given one
    WhileEq(&'a T),

    /// Run while the current unit is not equal to the given one
    WhileNot(&'a T),

    /// Run until the condition is true
    Until(&'a dyn Fn(&T) -> bool),

    /// Run until the current unit is equal to the given one
    UntilEq(&'a T),

    /// Run until the current unit is not equal to the given one
    UntilNot(&'a T),

    /// Run a fixed number of times
    Times(usize),

    /// Run until end and then go back n units
    Before(usize, Box<Self>)
}

impl<'a, T> LoopThrough<'a, T> {
    pub fn over<'b>(&self, cursor: &mut impl CursorNavigation<'b, Vec<T>, T>) -> usize where T: PartialEq + 'b {
        let mut count = 0;

        while let Some(unit) = cursor.peek() {
            match self {
                LoopThrough::All => {
                    cursor.next();
                    count += 1;
                },
                LoopThrough::While(condition) => {
                    if condition(unit) {
                        cursor.next();
                        count += 1;
                    } else {
                        break;
                    }
                },
                LoopThrough::WhileEq(expectation) => {
                    if &unit == expectation {
                        cursor.next();
                        count += 1;
                    } else {
                        break;
                    }
                },
                LoopThrough::WhileNot(expectation) => {
                    if &unit != expectation {
                        cursor.next();
                        count += 1;
                    } else {
                        break;
                    }
                },
                LoopThrough::Until(condition) => {
                    if condition(unit) {
                        break;
                    } else {
                        cursor.next();
                        count += 1;
                    }
                },
                LoopThrough::UntilEq(expectation) => {
                    if &unit == expectation {
                        break;
                    } else {
                        cursor.next();
                        count += 1;
                    }
                },
                LoopThrough::UntilNot(expectation) => {
                    if &unit != expectation {
                        break;
                    } else {
                        cursor.next();
                        count += 1;
                    }
                },
                LoopThrough::Times(amount) => {
                    cursor.move_by(*amount as isize);
                    count += *amount;
                    break;
                },
                LoopThrough::Before(amount, inner) => {
                    inner.over(cursor);

                    cursor.move_by(-(*amount as isize));
                    count = count.saturating_sub(*amount);
                    break;
                }
            }
        }

        count
    }
}