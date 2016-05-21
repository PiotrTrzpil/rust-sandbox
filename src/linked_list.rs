
type Link<T> = Option<Box<Item<T>>>;

struct Item<T> {
    value: T,
    next: Link<T>,
}

pub struct List<T> {
    head_item: Link<T>,
}

impl<T> List<T> {

    pub fn new() -> List<T> {
        List { head_item: None}
    }

    pub fn prepend(&mut self, i: T) {
        let new_item = Some(Box::new(Item {
            value: i,
            next: self.head_item.take(),
        }));
        self.head_item = new_item;
    }

    pub fn take_head(&mut self) -> Option<T> {
        self.head_item.take().map(|nodea| {
            let node = *nodea;
            self.head_item = node.next;
            node.value
        })
    }
}
