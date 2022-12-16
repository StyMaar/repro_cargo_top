use lower::foo;
use upper::get_x;

fn main() {
    let x = get_x();
    foo(x);
}

