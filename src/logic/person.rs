/*
 * Person module
 */

use super::{InstructionLevel, WorkingArea};

/// Person gender
#[derive(Debug, PartialEq)]
enum Gender {
    Male,
    Female,
}

/// An individual
pub struct Person {
    /// The person's name. Irrelevant, but might show in the news.
    name: String,

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
}

impl Person {
    /// Creates a new person, with zero days of life. A baby, pratically
    fn new(name: &str, gender: Gender, wished_area: WorkingArea) -> Person {
        Person {
            name: String::from(name),
            age: 0,
            gender,
            instruction_level: InstructionLevel::None,
            wished_area,
            working_area: None,
        }
    }

    /// Creates a new person, with a specified age and instruction level
    fn new_with_age(
        name: &str,
        gender: Gender,
        wished_area: WorkingArea,
        age: u64,
        instruction_level: InstructionLevel,
    ) -> Person {
        Person {
            name: String::from(name),
            age,
            gender,
            instruction_level,
            wished_area,
            working_area: None,
        }
    }

    /// Process one engine tick for this person
    /// One tick will mean one minute in-game, so 1440 ticks will mean a day
    ///
    /// The 'tick' parameter is the tick number we are currently in
    fn iterate(&mut self, tick: u64) {
        // Change the age.
        self.age = tick_to_day_number(tick);
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_person_creates_ok() {
        let p_test = Person::new("Test", Gender::Male, WorkingArea::Construction);
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
        let mut p_test = Person::new("Test", Gender::Male, WorkingArea::Construction);
        assert_eq!(0, p_test.age);

        for i in 0..(day_to_tick_number(1) + 1) {
            p_test.iterate(i);
        }

        assert_eq!(1, p_test.age);
    }

    #[test]
    fn test_person_passes_a_month() {
        let mut p_test = Person::new("Test", Gender::Male, WorkingArea::Construction);
        assert_eq!(0, p_test.age);

        for i in 0..(day_to_tick_number(30) + 1) {
            p_test.iterate(i);
        }

        assert_eq!(30, p_test.age);
    }

    #[test]
    fn test_person_passes_a_year() {
        let mut p_test = Person::new("Test", Gender::Male, WorkingArea::Construction);
        assert_eq!(0, p_test.age);

        for i in 0..(day_to_tick_number(365) + 1) {
            p_test.iterate(i);
        }

        assert_eq!(365, p_test.age);
    }
}
