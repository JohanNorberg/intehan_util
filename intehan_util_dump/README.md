# INTEHAN UTIL DUMP

Sortof like `dbg!` but easier to use and read.

## Example

```rust
use intehan_util_dump::dump;

fn main() {
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
```

prints:
```
[intehan_util_testbed/src/main.rs:5:5] [-] | foo = 1 |
[intehan_util_testbed/src/main.rs:6:5] [test one] | foo = 1 |
[intehan_util_testbed/src/main.rs:8:5] [test two] | foo = 1 | bar = 2 |
[intehan_util_testbed/src/main.rs:9:5] [-] | foo = 1 | bar = 2 |
[intehan_util_testbed/src/main.rs:11:5] [test two] | foo = 1 | bar = 2 | car = 3 |
[intehan_util_testbed/src/main.rs:12:5] [-] | foo = 1 | bar = 2 | car = 3 |
```
