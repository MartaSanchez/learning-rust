#![allow(dead_code)]

#[derive(Debug)]

pub enum Event {
    ActualFloor(i32),
    IsDoorOpen(bool),
    IsDoorClosed(bool),
    ButtonDirection(Direction),
    CarButton(i32),
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

pub fn car_arrived(floor: i32) -> Event {
    Event::ActualFloor(floor)
}

pub fn car_door_opened() -> Event {
    Event::IsDoorOpen(true)
}

pub fn car_door_closed() -> Event {
    Event::IsDoorClosed(true)
    
}

pub fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonDirection(dir);
    Event::ActualFloor(floor)
}

pub fn car_floor_button_pressed(floor: i32) -> Event {
    Event::CarButton(floor)
}

