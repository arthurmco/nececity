/*
 * Controls families
 */

use logic::person::*;
use std;
use std::collections::HashMap;

pub type FamilyID = usize;

/// Represents a family
///
/// A family is a collection of members.
/// We hold weak references, so we don't get an invalid reference if they die
/// or not (we delete death persons from the game memory), and because a people
/// can be part of two families: the one where it is the son/daughter and the
/// one where it is the father/mother
#[derive(Debug)]
pub struct Family {
    id: Option<FamilyID>,

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
            id: None,
            father: father.id.unwrap(),
            mother: mother.id.unwrap(),
            children: Vec::new(),
            age: 0,
        }
    }

    /// Creates an family with chldren
    pub fn new_with_children(father: &Person, mother: &Person, children: Vec<&Person>) -> Family {
        Family::new_with_children_and_age(father, mother, children, 0)
    }

    /// Creates an family with children
    fn new_with_children_and_age(
        father: &Person,
        mother: &Person,
        children: Vec<&Person>,
        age: u64,
    ) -> Family {
        // Also updates the references of all children]

        Family {
            id: None,
            father: father.id.unwrap(),
            mother: mother.id.unwrap(),
            children: children.iter().map(|c| c.id.unwrap()).collect(),
            age,
        }
    }

    /// Update children references, so that their family is now
    /// this actual family
    pub fn update_references(&self, list: &mut PersonList) {
        // Crash if we do not have an ID
        let id = self.id.unwrap();

        for c in &self.children {
            list.items
                .entry(*c)
                .and_modify(|e| e.update_original_family(id));
        }

        list.items
            .entry(self.father)
            .and_modify(|e| e.update_actual_family(id));
        list.items
            .entry(self.mother)
            .and_modify(|e| e.update_actual_family(id));
    }

    /// Do checks on this family
    fn iterate(&mut self) {}
}

/// A centralized list of families
pub struct FamilyList {
    pub items: HashMap<FamilyID, Family>,
    last_id: FamilyID,
}

impl FamilyList {
    pub fn new() -> FamilyList {
        FamilyList {
            items: HashMap::new(),
            last_id: 0,
        }
    }

    /// Add a person to the list. Returns an ID
    ///
    /// Note that the owner loses ownership to the person.
    /// It should now access it only through the list
    pub fn register(&mut self, p: Family) -> usize {
        let id = self.last_id + 1;

        let family = Family { id: Some(id), ..p };
        self.items.insert(id, family);

        self.last_id = id;

        id
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use logic::WorkingArea;

    #[test]
    fn test_family_create_children() {
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

    #[test]
    fn test_family_create_children_ref_family() {
        let common_attribs = PersonAttributes {
            intelligence: 255,
            beauty: 255,
            speak: 255,
            health: 255,
        };

        let mut plist = PersonList::new();
        let mut flist = FamilyList::new();

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

        let fid = flist.register(Family::new_with_children(
            &plist.items[&father],
            &plist.items[&mother],
            children.iter().map(|c| &plist.items[c]).collect(),
        ));
        let family = &flist.items[&fid];

        family.update_references(&mut plist);

        let father = &plist.items[&family.father];
        let mother = &plist.items[&family.father];
        let children = vec![
            &plist.items[&family.children[0]],
            &plist.items[&family.children[1]],
        ];

        assert_eq!(None, father.original_family);
        assert_eq!(None, mother.original_family);
        assert_eq!(Some(fid), father.actual_family);
        assert_eq!(Some(fid), mother.actual_family);
        assert_eq!(Some(fid), children[0].original_family);
        assert_eq!(Some(fid), children[1].original_family);
    }
}
