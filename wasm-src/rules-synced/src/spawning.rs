//! Storyteller spawn application, ported from `storyteller.lua`.

use crate::state::{DrillShip, Eskimo, Story, Units};
use crate::units::set_eskimo;

pub fn register_spawned(units: &mut Units, story: &mut Story, id: i32, name: &str) {
    match name {
        "coalburner" if !units.burners.contains(&id) => units.burners.push(id),
        "eskimo" => {
            story.story_eskimos += 1;
            if !units.eskimos.iter().any(|unit| unit.id == id) {
                units.eskimos.push(Eskimo {
                    id,
                    health: 100.0,
                    food: 90.0,
                    heat: 40.0,
                    eating: false,
                    warm: false,
                });
                set_eskimo(id, 100.0, 90.0, 40.0);
            }
        }
        "drillship" => {
            story.story_drills += 1;
            if !units.drills.iter().any(|ship| ship.id == id) {
                units.drills.push(DrillShip::new(id));
            }
        }
        "parachut" => {
            story.story_parachutes += 1;
            if !units.parachutes.contains(&id) {
                units.parachutes.push(id);
            }
        }
        "pirate" => {
            story.story_pirates += 1;
            if !units.pirates.contains(&id) {
                units.pirates.push(id);
            }
        }
        _ => {}
    }
}
