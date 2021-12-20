#![allow(clippy::all)]
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use itertools::Itertools;

struct Number {
    parent: Option<Rc<RefCell<Number>>>,
    value: usize,
    left: Option<Rc<RefCell<Number>>>,
    right: Option<Rc<RefCell<Number>>>,
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.left.as_ref().zip(self.right.as_ref()) {
            Some((left, right)) => write!(f, "[{:?},{:?}]", &*left.borrow(), &*right.borrow()),
            _ => write!(f, "{}", self.value),
        }
    }
}

fn parse_number(input: &str, parent: Option<Rc<RefCell<Number>>>) -> (&str, Rc<RefCell<Number>>) {
    match &input[0..1] {
        "[" => {
            assert_eq!(&input[0..1], "[");
            let (input, left) = parse_number(&input[1..], None);
            assert_eq!(&input[0..1], ",");
            let (input, right) = parse_number(&input[1..], None);
            assert_eq!(&input[0..1], "]");

            let this = Rc::new(RefCell::new(Number { left: Some(left.clone()), right: Some(right.clone()), parent, value: 0 }));

            left.borrow_mut().parent = Some(this.clone());
            right.borrow_mut().parent = Some(this.clone());
            (&input[1..], this)
        },
        s => {
            let n = s.chars().next().unwrap().to_digit(10).unwrap() as usize;
            (&input[1..], Rc::new(RefCell::new(Number {
                value: n,
                parent,
                left: None,
                right: None,
            })))
        }
    }
}

fn find_splitting_number(root: Rc<RefCell<Number>>) -> Option<Rc<RefCell<Number>>> {
    if root.borrow().value >= 10 {
        return Some(root)
    }
    if let Some(left) = &root.borrow().left {
        if let Some(root) = find_splitting_number(left.clone()) {
            return Some(root);
        }
    }
    if let Some(right) = &root.borrow().right {
        return find_splitting_number(right.clone())
    }
    None
}

fn find_exploding_number(root: Rc<RefCell<Number>>, depth: usize) -> Option<Rc<RefCell<Number>>> {
    if depth == 4 && root.borrow().left.is_some() {
        return Some(root)
    }
    if let Some(left) = &root.borrow().left {
        if let Some(root) = find_exploding_number(left.clone(), depth + 1) {
            return Some(root);
        }
    }
    if let Some(right) = &root.borrow().right {
        return find_exploding_number(right.clone(), depth + 1);
    }
    None
}
fn leftmost_node(node: Rc<RefCell<Number>>) -> Rc<RefCell<Number>> {
    if let Some(left) = &node.borrow().left {
        leftmost_node(left.clone())
    } else {
        node.clone()
    }
}

fn rightmost_node(node: Rc<RefCell<Number>>) -> Rc<RefCell<Number>> {
    if let Some(right) = &node.borrow().right {
        rightmost_node(right.clone())
    } else {
        node.clone()
    }
}

fn reduce(sn: Rc<RefCell<Number>>) {
    // println!("after addition: {:?}", &*sn.borrow());
    loop {
        if let Some(exploding) = find_exploding_number(sn.clone(), 0) {
            // Find first left parent
            let mut left_parent = None;
            let mut right_parent = None;

            let mut cur = exploding.clone();
            while left_parent.is_none() || right_parent.is_none() {
                let parent = cur.borrow().parent.clone();
                match parent {
                    Some(parent) => {
                        if Rc::ptr_eq(parent.borrow().left.as_ref().unwrap(), &cur) {
                            left_parent.get_or_insert(parent.clone());
                        }
                        if Rc::ptr_eq(parent.borrow().right.as_ref().unwrap(), &cur) {
                            right_parent.get_or_insert(parent.clone());
                        }
                        cur = parent;
                    }
                    None => break,
                }
            }

            let left_target = right_parent.and_then(|l| l.borrow().left.clone()).map(rightmost_node);
            let right_target = left_parent.and_then(|l| l.borrow().right.clone()).map(leftmost_node);

            if let Some(left_target) = left_target {
                left_target.borrow_mut().value += exploding.borrow().left.as_ref().unwrap().borrow().value;
            }
            if let Some(right_target) = right_target {
                right_target.borrow_mut().value += exploding.borrow().right.as_ref().unwrap().borrow().value;
            }
            exploding.borrow_mut().value = 0;
            exploding.borrow_mut().left = None;
            exploding.borrow_mut().right = None;
        } else if let Some(splitting_number) = find_splitting_number(sn.clone()) {
            let value = splitting_number.borrow().value;
            let left_value = value / 2;
            let right_value = value / 2 + value % 2;

            let left = Rc::new(RefCell::new(Number {
                value: left_value,
                parent: Some(splitting_number.clone()),
                left: None,
                right: None,
            }));
            let right = Rc::new(RefCell::new(Number {
                value: right_value,
                parent: Some(splitting_number.clone()),
                left: None,
                right: None,
            }));
            splitting_number.borrow_mut().value = 0;
            splitting_number.borrow_mut().left = Some(left);
            splitting_number.borrow_mut().right = Some(right);
            // println!("after split: {:?}", &*sn.borrow());
        } else {
            break;
        }
    }
}

fn magnitude(number: Rc<RefCell<Number>>) -> usize {
    match number.borrow().left.as_ref().zip(number.borrow().right.as_ref()) {
        None => number.borrow().value,
        Some((left, right)) => {
            magnitude(left.clone()) * 3 + magnitude(right.clone()) * 2
        }
    }
}

pub fn part1() {
    let mut numbers = include_str!("input").lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            parse_number(l, None).1
        })
        .collect_vec();

    let initial = numbers[0].clone();

    let result = (&numbers[1..]).iter().fold(initial, |acc, number| {
        let new_number = Rc::new(RefCell::new(Number {
            parent: None,
            value: 0,
            left: Some(acc.clone()),
            right: Some(number.clone()),
        }));

        number.borrow_mut().parent = Some(new_number.clone());
        acc.borrow_mut().parent = Some(new_number.clone());
        reduce(new_number.clone());
        new_number
    });
    dbg!(magnitude(result));

    let mut max_magnitude = 0;
    for i in 0..100 {
        for j in 0..100 {
            if i == j {
                continue;
            }
            // FML
            let mut numbers = include_str!("input").lines()
                .filter(|l| !l.is_empty())
                .map(|l| {
                    parse_number(l, None).1
                })
                .collect_vec();
            let new_number = Rc::new(RefCell::new(Number {
                parent: None,
                value: 0,
                left: Some(numbers[i].clone()),
                right: Some(numbers[j].clone()),
            }));
            numbers[i].borrow_mut().parent = Some(new_number.clone());
            numbers[j].borrow_mut().parent = Some(new_number.clone());
            reduce(new_number.clone());
            max_magnitude = std::cmp::max(max_magnitude, magnitude(new_number));
        }
    }
    dbg!(max_magnitude);
}
