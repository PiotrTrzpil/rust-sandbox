
mod linked_list;

use linked_list::List;

fn main() {
    let mut a: List<i32> = List::new();
    a.prepend(45);
    let ret = a.take_head();
    println!("{:?}", ret);
}
