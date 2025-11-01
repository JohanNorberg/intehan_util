#[macro_export]
macro_rules! dump {

    // One thing
    ($val:expr) => {
        match $val {
            tmp => {
                eprintln!(
                    "[{}:{}:{}] [-] | {} = {:?} |",
                    file!(),
                    line!(),
                    column!(),
                    stringify!($val),
                    &tmp
                );
                tmp
            }
        }
    };

    // Single expression (after the required string)
    ($msg:literal, $val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!(
                    "[{}:{}:{}] [{}] | {} = {:?} |",
                    file!(),
                    line!(),
                    column!(),
                    $msg,
                    stringify!($val),
                    &tmp
                );
                tmp
            }
        }
    };

    // Multiple expressions (after the required string)
    ($msg:literal, $($val:expr),+ $(,)?) => {
        {
            eprint!(
                "[{}:{}:{}] [{}] |",
                file!(),
                line!(),
                column!(),
                $msg,
            );
            (
                $(
                    match $val {
                        tmp => {
                            eprint!(
                                " {} = {:?} |",
                                stringify!($val),
                                &tmp
                            );
                            tmp
                        }
                    }
                ),+
            );
            eprintln!();
        }
    };

    // Multiple expressions, no string
    ($($val:expr),+ $(,)?) => {
        {
            eprint!(
                "[{}:{}:{}] [-] |",
                file!(),
                line!(),
                column!(),
            );
            (
                $(
                    match $val {
                        tmp => {
                            eprint!(
                                " {} = {:?} |",
                                stringify!($val),
                                &tmp
                            );
                            tmp
                        }
                    }
                ),+
            );
            eprintln!();
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let foo = 1;
        dump!(foo);
        dump!("test one", foo);
        let bar = 2;
        dump!("test two", foo, bar);
        dump!(foo, bar);
        let car = 3;
        dump!("test two", foo, bar, car);
        dump!(foo, bar, car);
    }
}
