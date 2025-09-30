#[macro_export]
macro_rules! dump {
    // Single expression (after the required string)
    ($msg:expr, $val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!(
                    "[{}:{}:{}] [{}] {} = {:?}",
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
    ($msg:expr, $($val:expr),+ $(,)?) => {
        {
            eprint!(
                "[{}:{}:{}] [{}]",
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
                                " {} = {:?}",
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
        dump!("test one", foo);
        let bar = 2;
        dump!("test two", foo, bar);
    }
}
