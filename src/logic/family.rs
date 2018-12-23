/*
 * Controls families
 */

use logic::person::*;
use std;

/// Represents a family
///
/// A family is a collection of members.
/// We hold weak references, so we don't get an invalid reference if they die
/// or not (we delete death persons from the game memory)
pub struct Family {
    father: std::rc::Weak<Person>,
    mother: std::rc::Weak<Person>,
    children: Vec<std::rc::Weak<Person>>,

    /// Time of existence of this family
    age: u64,
}

impl Family {
    /// Creates a planned family. One man and one woman
    fn new(father: std::rc::Weak<Person>, mother: std::rc::Weak<Person>) -> Family {
        Family {
            father,
            mother,
            children: Vec::new(),
            age: 0,
        }
    }

    /// Creates an family with chldren
    fn new_with_children(
        father: std::rc::Weak<Person>,
        mother: std::rc::Weak<Person>,
        children: Vec<std::rc::Weak<Person>>,
    ) -> Family {
        Family {
            father,
            mother,
            children,
            age: 0,
        }
    }

    /// Creates an family with children
    fn new_with_children_and_age(
        father: std::rc::Weak<Person>,
        mother: std::rc::Weak<Person>,
        children: Vec<std::rc::Weak<Person>>,
        age: u64,
    ) -> Family {
        Family {
            father,
            mother,
            children,
            age,
        }
    }

    /// Do checks on this family
    fn iterate(&mut self) {}
}
