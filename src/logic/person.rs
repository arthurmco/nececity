/*
 * Person module
 */

use super::{InstructionLevel, WorkingArea};
use logic::family::{Family, FamilyID};
use std;
use std::collections::HashMap;

/// Person gender
#[derive(Debug, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

/// Store person attributes
///
/// Those attributes can go from 1 to 255
/// This will be treated like a C plain old data type: just a bunch of
/// aggregated values. Because of this, we'll implement the copy and
/// clone traits
#[derive(Debug, Copy, Clone)]
pub struct PersonAttributes {
    /// Intelligence level (affects how fast the person learns, and how much)
    pub intelligence: u8,

    /// Beauty level (affects how faster the person can find a partner)
    pub beauty: u8,

    /// Speak level (affects how famous the person can be, and how fast you get a job)
    pub speak: u8,

    /// Health level (affect how hard it is for the person to get a disease)
    pub health: u8,
}

pub type PersonID = usize;

/// An individual
#[derive(Debug)]
pub struct Person {
    /// The person ID.
    /// Some(...) if the value is in a list, None if it is not
    pub id: Option<PersonID>,

    /// The person's name. Irrelevant, but might show in the news.
    pub name: String,

    /// Person age, in days.
    age: u64,

    /// The person gender
    gender: Gender,

    /// Person current instruction level
    instruction_level: InstructionLevel,

    /// Area that the person likes, and wishes to work in
    wished_area: WorkingArea,

    /// Area that the person currently works in. Can be none, too.
    working_area: Option<WorkingArea>,

    /// Person attributes
    attributes: PersonAttributes,

    /// Alive or dead?
    _is_alive: bool,

    /// Original family (the one with its father and mother
    pub original_family: Option<FamilyID>,

    /// Actual family where it lives in
    pub actual_family: Option<FamilyID>,
}

impl Person {
    /// Creates a new person, with zero days of life. A baby, pratically
    pub fn new(
        name: &str,
        gender: Gender,
        wished_area: WorkingArea,
        attributes: PersonAttributes,
    ) -> Person {
        Person {
            id: None,
            name: String::from(name),
            age: 0,
            gender,
            instruction_level: InstructionLevel::None,
            wished_area,
            working_area: None,
            attributes,
            _is_alive: true,
            original_family: None,
            actual_family: None,
        }
    }

    /// Creates a new person, with a specified age and instruction level
    pub fn new_with_age(
        name: &str,
        gender: Gender,
        wished_area: WorkingArea,
        attributes: PersonAttributes,
        age: u64,
        instruction_level: InstructionLevel,
    ) -> Person {
        Person {
            id: None,
            name: String::from(name),
            age,
            gender,
            instruction_level,
            wished_area,
            working_area: None,
            attributes,
            _is_alive: true,
            original_family: None,
            actual_family: None,
        }
    }

    /// Check if the person is alive or not
    fn is_alive(&self) -> bool {
        self._is_alive
    }

    /// Update original family
    pub fn update_original_family(&mut self, f: FamilyID) {
        self.original_family = Some(f);
    }

    /// Update original family
    pub fn update_actual_family(&mut self, f: FamilyID) {
        self.actual_family = Some(f);
    }

    /// Process one engine tick for this person
    /// One tick will mean one minute in-game, so 1440 ticks will mean a day
    ///
    /// The 'tick' parameter is the tick number we are currently in
    fn iterate(&mut self, tick: u64) {
        // Change the person age.
        self.age = tick_to_day_number(tick);

        const MAX_LIFE_YEARS: u64 = 110;
        // Death age is proportional to the health levels
        // More bigger the health levels are, less chance the person has of getting
        // sick
        // Like the real world, when you are too sick, you die.
        // And you die of old age too. In this world, when you make 110 years
        //
        // People start dying with 50 years.
        // TODO: Make this value dependant on the city?
        if self.age >= 50 * 365 {
            let health = self.attributes.health;
            if self.age
                >= (50.0 + (((health as f64) / 255.0) * (MAX_LIFE_YEARS - 50) as f64)) as u64 * 365
            {
                self._is_alive = false;
            }
        }
    }
}

/// Convert tick number to day number, in integer
fn tick_to_day_number(tick: u64) -> u64 {
    tick / 1440
}

/// Convert day number to tick number, in integer
fn day_to_tick_number(day: u64) -> u64 {
    day * 1440
}

/// A centralized list of persons
pub struct PersonList {
    pub items: HashMap<PersonID, Person>,
    last_id: PersonID,
}

impl PersonList {
    pub fn new() -> PersonList {
        PersonList {
            items: HashMap::new(),
            last_id: 0,
        }
    }

    /// Add a person to the list. Returns an ID
    ///
    /// Note that the owner loses ownership to the person.
    /// It should now access it only through the list
    pub fn register(&mut self, p: Person) -> usize {
        let id = self.last_id + 1;

        let person = Person { id: Some(id), ..p };
        self.items.insert(id, person);

        self.last_id = id;

        id
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_person_creates_ok() {
        let p_test = Person::new(
            "Test",
            Gender::Male,
            WorkingArea::Construction,
            PersonAttributes {
                intelligence: 0,
                beauty: 0,
                speak: 0,
                health: 0,
            },
        );
        assert_eq!("Test", p_test.name);
        assert_eq!(0, p_test.age);
        assert_eq!(Gender::Male, p_test.gender);
        assert_eq!(InstructionLevel::None, p_test.instruction_level);
        assert_eq!(WorkingArea::Construction, p_test.wished_area);
        assert_eq!(Option::None, p_test.working_area);
    }

    #[test]
    fn test_person_creates_ok_with_age() {
        let p_test = Person::new_with_age(
            "Test2",
            Gender::Female,
            WorkingArea::Construction,
            PersonAttributes {
                intelligence: 0,
                beauty: 0,
                speak: 0,
                health: 0,
            },
            1000,
            InstructionLevel::Advanced,
        );
        assert_eq!("Test2", p_test.name);
        assert_eq!(1000, p_test.age);
        assert_eq!(Gender::Female, p_test.gender);
        assert_eq!(InstructionLevel::Advanced, p_test.instruction_level);
        assert_eq!(WorkingArea::Construction, p_test.wished_area);
        assert_eq!(Option::None, p_test.working_area);
    }

    #[test]
    fn test_person_passes_a_day() {
        let mut p_test = Person::new(
            "Test",
            Gender::Male,
            WorkingArea::Construction,
            PersonAttributes {
                intelligence: 0,
                beauty: 0,
                speak: 0,
                health: 0,
            },
        );
        assert_eq!(0, p_test.age);

        for i in 0..(day_to_tick_number(1) + 1) {
            p_test.iterate(i);
        }

        assert_eq!(1, p_test.age);
    }

    #[test]
    fn test_person_passes_a_month() {
        let mut p_test = Person::new(
            "Test",
            Gender::Male,
            WorkingArea::Construction,
            PersonAttributes {
                intelligence: 0,
                beauty: 0,
                speak: 0,
                health: 0,
            },
        );
        assert_eq!(0, p_test.age);

        for i in 0..(day_to_tick_number(30) + 1) {
            p_test.iterate(i);
        }

        assert_eq!(30, p_test.age);
    }

    #[test]
    fn test_person_passes_a_year() {
        let mut p_test = Person::new(
            "Test",
            Gender::Male,
            WorkingArea::Construction,
            PersonAttributes {
                intelligence: 0,
                beauty: 0,
                speak: 0,
                health: 0,
            },
        );
        assert_eq!(0, p_test.age);

        for i in 0..(day_to_tick_number(365) + 1) {
            p_test.iterate(i);
        }

        assert_eq!(365, p_test.age);
    }

    #[test]
    fn test_person_too_old_to_keep_living() {
        let mut p_test = Person::new(
            "Test",
            Gender::Male,
            WorkingArea::Construction,
            PersonAttributes {
                intelligence: 0,
                beauty: 0,
                speak: 0,
                health: 255,
            },
        );

        for i in 0..(day_to_tick_number(365 * 110) + 1) {
            p_test.iterate(i);
        }

        assert_eq!(false, p_test.is_alive());
    }

    #[test]
    fn test_person_start_of_death_date() {
        let mut p_test = Person::new(
            "Test",
            Gender::Male,
            WorkingArea::Construction,
            PersonAttributes {
                intelligence: 0,
                beauty: 0,
                speak: 0,
                health: 255,
            },
        );

        for i in 0..(day_to_tick_number(365 * 50) + 1) {
            p_test.iterate(i);
        }

        assert_eq!(true, p_test.is_alive());
    }

    #[test]
    fn test_person_low_health_level() {
        let mut p_test = Person::new(
            "Test",
            Gender::Male,
            WorkingArea::Construction,
            PersonAttributes {
                intelligence: 0,
                beauty: 0,
                speak: 0,
                health: 10,
            },
        );

        // People with low health levels should die soon
        for i in 0..(day_to_tick_number(365 * 55) + 1) {
            p_test.iterate(i);
        }

        assert_eq!(false, p_test.is_alive());
    }

    #[test]
    fn test_person_high_health_level() {
        let mut p_test = Person::new(
            "Test",
            Gender::Male,
            WorkingArea::Construction,
            PersonAttributes {
                intelligence: 0,
                beauty: 0,
                speak: 0,
                health: 90,
            },
        );

        // People with high health levels should not die soon
        for i in 0..(day_to_tick_number(365 * 70) + 1) {
            p_test.iterate(i);
        }

        assert_eq!(true, p_test.is_alive());
    }

    #[test]
    fn test_person_high_health_but_not_enough_level() {
        let mut p_test = Person::new(
            "Test",
            Gender::Male,
            WorkingArea::Construction,
            PersonAttributes {
                intelligence: 0,
                beauty: 0,
                speak: 0,
                health: 90,
            },
        );

        // People with high health levels should not die soon
        for i in 0..(day_to_tick_number(365 * 95) + 1) {
            p_test.iterate(i);
        }

        assert_eq!(false, p_test.is_alive());
    }

    #[test]
    fn test_person_very_high_health_level() {
        let mut p_test = Person::new(
            "Test",
            Gender::Male,
            WorkingArea::Construction,
            PersonAttributes {
                intelligence: 0,
                beauty: 0,
                speak: 0,
                health: 212,
            },
        );

        // People with high health levels should not die soon
        for i in 0..(day_to_tick_number(365 * 95) + 1) {
            p_test.iterate(i);
        }

        assert_eq!(true, p_test.is_alive());
    }
}
