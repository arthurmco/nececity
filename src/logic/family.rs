/*
 * Controls families
 */

use logic::person::*;
use std;

/// Represents a family
///
/// A family is a collection of members.
/// We hold weak references, so we don't get an invalid reference if they die
/// or not (we delete death persons from the game memory), and because a people
/// can be part of two families: the one where it is the son/daughter and the
/// one where it is the father/mother
#[derive(Debug)]
pub struct Family {
    father: PersonID,
    mother: PersonID,

    children: Vec<PersonID>,

    /// Time of existence of this family
    age: u64,
}

impl Family {
    /// Creates a planned family. One man and one woman
    pub fn new(father: &Person, mother: &Person) -> Family {
        Family {
            father: father.id.unwrap(),
            mother: mother.id.unwrap(),
            children: Vec::new(),
            age: 0,
        }
    }

    /// Creates an family with chldren
    pub fn new_with_children(
        father: &Person,
        mother: &Person,
        children: Vec<&Person>,
    ) -> std::rc::Rc<Family> {
        let mut family = std::rc::Rc::new(Family {
            father: father.id.unwrap(),
            mother: mother.id.unwrap(),
            children: children.iter().map(|c| c.id.unwrap()).collect(),
            age: 0,
        });

        family
    }

    /// Creates an family with children
    fn new_with_children_and_age(
        father: &Person,
        mother: &Person,
        children: Vec<&Person>,
        age: u64,
    ) -> std::rc::Rc<Family> {
        std::rc::Rc::new(Family {
            father: father.id.unwrap(),
            mother: mother.id.unwrap(),
            children: children.iter().map(|c| c.id.unwrap()).collect(),
            age,
        })
    }

    /// Do checks on this family
    fn iterate(&mut self) {}
}

#[cfg(test)]
mod tests {

    use super::*;
    use logic::WorkingArea;

    #[test]
    fn test_family_create_children_ref_family() {
        let common_attribs = PersonAttributes {
            intelligence: 255,
            beauty: 255,
            speak: 255,
            health: 255,
        };

        let mut plist = PersonList::new();
        let father = plist.register(Person::new(
            "Father",
            Gender::Male,
            WorkingArea::Education,
            common_attribs,
        ));
        let mother = plist.register(Person::new(
            "Mother",
            Gender::Female,
            WorkingArea::Education,
            common_attribs,
        ));

        let children = vec![
            plist.register(Person::new(
                "Child 1",
                Gender::Male,
                WorkingArea::Education,
                common_attribs,
            )),
            plist.register(Person::new(
                "Child 2",
                Gender::Female,
                WorkingArea::Education,
                common_attribs,
            )),
        ];

        let family = Family::new_with_children(
            &plist.items[&father],
            &plist.items[&mother],
            children.iter().map(|c| &plist.items[c]).collect(),
        );

        assert_eq!("Father", plist.items[&family.father].name);
        assert_eq!("Mother", plist.items[&family.mother].name);
        assert_eq!("Child 1", plist.items[&family.children[0]].name);
        assert_eq!("Child 2", plist.items[&family.children[1]].name);
    }
}
