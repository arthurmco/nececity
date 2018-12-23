/*
 * The logic part of our game
 *
 */

/// Working areas of each person and place.
///
/// A person specialized some area will need a job for that area.
/// If it can't find for a period of time, it will move out of your city.
#[derive(Debug, PartialEq)]
pub enum WorkingArea {
    Education,
    Health,
    Technology,
    Construction,
    Driving,
    Homecare,
}

/* TODO: Add more working areas */

/// The instruction level, for jobs and persons
///
/// The idea is that a job of some instruction level needs a person of the same or higher
/// level
#[derive(Debug, PartialEq)]
pub enum InstructionLevel {
    /// No instruction. Babies and young children fall here
    None,

    /// Basic instruction level. No school required, but you need to have at least 6 years
    Basic,

    /// Intermediate instruction level. Only basic school required
    Intermediate,

    /// Technical instruction level. Only come from technical schools and above
    Technical,

    /// Advanced instruction level. Only come from universities
    Advanced,

    /// Experience in some field. Only works if you already works
    ///
    /// Receives a working area and an amount of months
    Experience(WorkingArea, i32),
}


pub mod person;
pub mod family;
