use std::{
    cell::{Cell, RefCell},
    num::ParseIntError,
    ops::Add,
    rc::{Rc, Weak},
    str::FromStr,
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
    Leaf(Rc<Cell<i32>>),
    Pair(Rc<RefCell<Inner>>),
}

impl From<i32> for Element {
    fn from(num: i32) -> Self {
        Element::Leaf(Rc::new(Cell::new(num)))
    }
}

impl From<Number> for Element {
    fn from(num: Number) -> Self {
        Element::Pair(num.inner)
    }
}

impl Element {
    fn child_pair(&self) -> Option<Rc<RefCell<Inner>>> {
        if let Element::Pair(child) = self {
            Some(child.clone())
        } else {
            None
        }
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

    fn left_pair(&self) -> Option<Self> {
        Inner::left_pair(&self.inner).map(|inner| Number { inner })
    }

    fn right_pair(&self) -> Option<Self> {
        Inner::right_pair(&self.inner).map(|inner| Number { inner })
    }

    fn reduce(&mut self) {
        fn try_explode(this: Rc<RefCell<Inner>>, nested: usize) -> bool {
            dbg!(nested, this.borrow().to_string());

            if nested == 4 {
                if let Element::Leaf(left) = &this.borrow().left {
                    let left_leaf = Inner::first_leaf_to_the_left(&this);
                    dbg!(left_leaf.get());
                    left_leaf.set(left_leaf.get() + left.get());
                } else {
                    unreachable!();
                }

                if let Element::Leaf(right) = &this.borrow().right {
                    let right_leaf = Inner::first_leaf_to_the_right(&this);
                    dbg!(right_leaf.get());
                    right_leaf.set(right_leaf.get() + right.get());
                } else {
                    unreachable!();
                }

                if let Some(parent) = this.borrow().parent.upgrade() {
                    let mut parent = parent.borrow_mut();

                    if matches!(&parent.left, Element::Pair(left) if Rc::ptr_eq(left, &this)) {
                        parent.left = Element::from(0);
                    } else {
                        parent.right = Element::from(0);
                    }
                } else {
                    unreachable!();
                }

                true
            } else {
                let children = [Inner::left_pair(&this), Inner::right_pair(&this)];

                for child in children.into_iter().flatten() {
                    if try_explode(child, nested + 1) {
                        return true;
                    }
                }

                false
            }
        }

        fn try_split(this: Rc<RefCell<Inner>>) -> bool {
            fn try_split_element(element: &mut Element, parent: &Weak<RefCell<Inner>>) -> bool {
                match element {
                    Element::Leaf(num) => {
                        let num = num.get();
                        if num >= 10 {
                            let new_left = (num as f64 / 2.).floor() as i32;
                            let new_right = (num as f64 / 2.).ceil() as i32;
                            let new_pair = Inner {
                                left: Element::from(new_left),
                                right: Element::from(new_right),
                                parent: parent.clone(),
                            };
                            *element = Element::Pair(Rc::new(RefCell::new(new_pair)));
                            return true;
                        }
                    }
                    Element::Pair(child) => {
                        let weak = Rc::downgrade(child);
                        let mut child = child.borrow_mut();
                        if try_split_element(&mut child.left, &weak)
                            || try_split_element(&mut child.right, &weak)
                        {
                            return true;
                        }
                    }
                }

                false
            }

            dbg!(this.borrow().to_string());

            let root = Weak::new();
            let mut this = this.borrow_mut();
            try_split_element(&mut this.left, &root) || try_split_element(&mut this.right, &root)
        }

        while try_explode(self.inner.clone(), 0) || try_split(self.inner.clone()) {}
    }
}

impl FromStr for Number {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_leaf(s: &str) -> Result<(Element, &str), ParseIntError> {
            s[0..1].parse::<i32>().map(|num| (num.into(), &s[1..]))
        }

        fn parse_pair(s: &str) -> Result<(Element, &str), &'static str> {
            let s = s.strip_prefix('[').ok_or("missing opening bracket")?;
            let (left, s) = parse_leaf(s).or_else(|_| parse_pair(s))?;
            let s = s.strip_prefix(',').ok_or("missing comma")?;
            let (right, s) = parse_leaf(s).or_else(|_| parse_pair(s))?;
            let s = s.strip_prefix(']').ok_or("missing closing bracket")?;
            Ok((Number::new(left, right).into(), s))
        }

        let (pair, rem) = parse_pair(s)?;

        if !rem.is_empty() {
            return Err("trailing characters");
        }

        if let Element::Pair(inner) = pair {
            Ok(Number { inner })
        } else {
            unreachable!()
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Number::new(self.into(), rhs.into());
        res.reduce();
        res
    }
}

impl Inner {
    fn leftmost_leaf(this: &Rc<RefCell<Self>>) -> Rc<Cell<i32>> {
        match &this.borrow().left {
            Element::Leaf(num) => num.clone(),
            Element::Pair(num) => Inner::leftmost_leaf(num),
        }
    }

    fn first_leaf_to_the_left(this: &Rc<RefCell<Self>>) -> Rc<Cell<i32>> {
        let parent = &this.borrow().parent.upgrade().expect("already at the root");
        Inner::leftmost_leaf(parent)
    }

    fn rightmost_leaf(this: &Rc<RefCell<Self>>) -> Rc<Cell<i32>> {
        match &this.borrow().right {
            Element::Leaf(num) => num.clone(),
            Element::Pair(num) => Inner::rightmost_leaf(num),
        }
    }

    fn first_leaf_to_the_right(this: &Rc<RefCell<Self>>) -> Rc<Cell<i32>> {
        let parent = &this.borrow().parent.upgrade().expect("already at the root");
        Inner::rightmost_leaf(parent)
    }

    fn left_pair(this: &Rc<RefCell<Self>>) -> Option<Rc<RefCell<Self>>> {
        this.borrow().left.child_pair()
    }

    fn right_pair(this: &Rc<RefCell<Self>>) -> Option<Rc<RefCell<Self>>> {
        this.borrow().right.child_pair()
    }
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
            Element::Leaf(num) => num.get().fmt(f),
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

    #[test]
    fn finds_the_first_leaf_to_the_right() {
        let inner = Number::new(1.into(), 2.into());
        let outer = Number::new(inner.into(), 3.into());

        let right_leaf = Inner::first_leaf_to_the_right(&outer.left_pair().unwrap().inner);
        assert_eq!(right_leaf.get(), 3);
    }

    #[test]
    fn finds_the_first_leaf_to_the_left() {
        let inner = Number::new(2.into(), 3.into());
        let outer = Number::new(1.into(), inner.into());

        let left_leaf = Inner::first_leaf_to_the_left(&outer.right_pair().unwrap().inner);
        assert_eq!(left_leaf.get(), 1);
    }

    #[test]
    fn parses() {
        let s = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let num: Number = s.parse().unwrap();
        assert_eq!(num.to_string(), s);
    }

    fn n(s: &str) -> Number {
        s.parse().expect("could not parse")
    }

    #[test]
    fn does_simple_additions() {
        let lhs = n("[1,2]");
        let rhs = n("[[3,4],5]");
        let expected = n("[[1,2],[[3,4],5]]");
        assert_eq!((lhs + rhs).to_string(), expected.to_string());
    }

    #[test]
    fn does_basic_explosions() {
        let mut num = n("[[[[[9,8],1],2],3],4]");
        num.reduce();
        assert_eq!(num.to_string(), n("[[[[0,9],2],3],4]").to_string());
    }

    #[test]
    fn does_additions_with_reductions() {
        let lhs = n("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let rhs = n("[1,1]");
        let expected = n("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!((lhs + rhs).to_string(), expected.to_string());
    }
}
