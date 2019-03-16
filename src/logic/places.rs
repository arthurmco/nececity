/*
 * Places
 */

use logic::family::*;
use logic::person::*;
use std;


/// Place type
#[derive(Debug, PartialEq)]
pub enum PlaceType {
    Work,
    Leisure,
    Commerce
}


/// Represents a place
///
/// A place is some location where two or more families can go.
///
/// You can go there with your family, your friends or alone.
/// It depends of your
pub struct Place {
    name: String,

    /// Place age, in days.
    /// Can influence popularity (people often prefer new things, or too old things)
    age: u64

    type: PlaceType,

    /// People there
    people: std::rc::Weak<Person>
}
