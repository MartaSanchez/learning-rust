#![allow(dead_code)]

#[derive(Debug)]

pub enum Event {
    // TODO: add required variants
    ActualFloor(i32),
    IsDoorOpen(bool),
    IsDoorClosed(bool),
    ButtonDirection(Direction),
    CarButton(i32),
}

/// A direction of travel.
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

/// The car has arrived on the given floor.
pub fn car_arrived(floor: i32) -> Event {
    Event::ActualFloor(floor)
}

/// The car doors have opened.
pub fn car_door_opened() -> Event {
    Event::IsDoorOpen(true)
}

/// The car doors have closed.
pub fn car_door_closed() -> Event {
    Event::IsDoorClosed(true)
    
}

/// A directional button was pressed in an elevator lobby on the given floor.
pub fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonDirection(dir);
    Event::ActualFloor(floor)
}

/// A floor button was pressed in the elevator car.
pub fn car_floor_button_pressed(floor: i32) -> Event {
    Event::CarButton(floor)
}

