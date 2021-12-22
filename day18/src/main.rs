use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

fn main() {
    println!("--- Day 18: Snailfish ---");
}

#[derive(Debug)]
struct Number {
    inner: Rc<RefCell<Inner>>,
}

#[derive(Debug)]
struct Inner {
    parent: Weak<RefCell<Inner>>,
    left: Element,
    right: Element,
}

#[derive(Debug)]
enum Element {
    Leaf(i32),
    Pair(Rc<RefCell<Inner>>),
}

impl From<i32> for Element {
    fn from(num: i32) -> Self {
        Element::Leaf(num)
    }
}

impl From<Number> for Element {
    fn from(num: Number) -> Self {
        Element::Pair(num.inner)
    }
}

impl Number {
    fn new(left: Element, right: Element) -> Self {
        let inner = Rc::new(RefCell::new(Inner {
            parent: Weak::new(),
            left,
            right,
        }));

        if let Element::Pair(left) = &inner.borrow().left {
            left.borrow_mut().parent = Rc::downgrade(&inner);
        }

        if let Element::Pair(right) = &inner.borrow().right {
            right.borrow_mut().parent = Rc::downgrade(&inner);
        }

        Number { inner }
    }
}

impl Inner {
    // fn leftmost_leaf(self: Rc<Self>) -> Rc<Self> {
    //     match &self.element {
    //         Element::Leaf(_) => self,
    //         Element::Pair { left, .. } => Rc::clone(left).leftmost_leaf(),
    //     }
    // }

    // fn rightmost_leaf(self: Rc<Self>) -> Rc<Self> {
    //     match &self.element {
    //         Element::Leaf(_) => self,
    //         Element::Pair { right, .. } => Rc::clone(right).rightmost_leaf(),
    //     }
    // }

    // fn first_left_left(self: Rc<Self>) -> Rc<Self> {
    //     self.parent
    //         .upgrade()
    //         .expect("already at the root")
    //         .rightmost_leaf()
    // }

    // fn first_right_right(self: Rc<Self>) -> Rc<Self> {
    //     self.parent
    //         .upgrade()
    //         .expect("already at the root")
    //         .leftmost_leaf()
    // }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.borrow().fmt(f)
    }
}

impl std::fmt::Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{},{}]", self.left, self.right,))
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Element::Leaf(num) => num.fmt(f),
            Element::Pair(num) => num.borrow().fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_constructed_and_printed() {
        let inner = Number::new(1.into(), 2.into());
        assert_eq!(format!("{}", inner), "[1,2]");

        let outer = Number::new(inner.into(), 3.into());
        assert_eq!(format!("{}", outer), "[[1,2],3]");
    }

    #[test]
    fn parents_are_correctly_set() {
        let left_pair = Number::new(1.into(), 2.into());
        let right_pair = Number::new(3.into(), 4.into());
        let outer = Number::new(left_pair.into(), right_pair.into());

        let mut left_parent = None;
        let mut right_parent = None;

        if let Element::Pair(left) = &outer.inner.borrow().left {
            left_parent = Some(left.borrow().parent.clone());
        };

        if let Element::Pair(right) = &outer.inner.borrow().right {
            right_parent = Some(right.borrow().parent.clone());
        };

        let expected = Rc::downgrade(&outer.inner);

        assert!(left_parent.unwrap().ptr_eq(&expected));
        assert!(right_parent.unwrap().ptr_eq(&expected));
    }
}
