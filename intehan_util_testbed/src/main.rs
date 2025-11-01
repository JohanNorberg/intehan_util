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
