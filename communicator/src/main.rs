extern crate communicator;

enum TrafficLight {
  Red,
  Yellow,
  Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
  communicator::client::connect();

  let _red = Red;
  let _yellow = Yellow;
  let _green = TrafficLight::Green;
}