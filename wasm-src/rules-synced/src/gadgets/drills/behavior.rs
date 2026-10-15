use alloc::vec::Vec;

use spring::consts::SQUARE_SIZE;

use crate::terrain::{self, ICE_HEIGHT, round_to_square};

#[derive(Clone, Copy)]
pub struct DrillShip {
    pub id: i32,
    created_frame: i32,
    mine_piece: Option<i32>,
    pub order: Option<(f32, f32)>,
    checked_frame: i32,
}

#[derive(Default)]
pub struct Drills {
    pub ships: Vec<DrillShip>,
    out_of_ice: bool,
}

const CHECK_INTERVAL: i32 = 30;
const DRILL_PERIOD: i32 = 15;
const DRILL_SIZE: i32 = 250;
const DRILL_AMOUNT: f32 = 2.0;

impl DrillShip {
    pub fn new(id: i32, created_frame: i32) -> Self {
        let mine_piece = spring::get_unit_piece_map(id).ok().and_then(|map| {
            map.iter()
                .find(|entry| entry.name == "icemine")
                .map(|entry| entry.piece_num)
        });
        Self {
            id,
            created_frame,
            mine_piece,
            order: None,
            checked_frame: -1,
        }
    }
}

pub fn drill(drills: &Drills, frame: i32) {
    for ship in &drills.ships {
        let age = frame - ship.created_frame;
        if age <= 0 || age % DRILL_PERIOD != 0 {
            continue;
        }
        let tip = ship
            .mine_piece
            .and_then(|piece| spring::get_unit_piece_pos_dir(ship.id, piece).ok())
            .map(|pos_dir| pos_dir.position)
            .or_else(|| spring::get_unit_position(ship.id, false, false).ok());
        if let Some(tip) = tip {
            terrain::decrease_terrain(tip.x, tip.z, DRILL_SIZE, DRILL_AMOUNT);
        }
    }
}

pub fn update_drills(drills: &mut Drills, frame: i32) {
    if drills.out_of_ice {
        return;
    }
    for ship in &mut drills.ships {
        if frame - ship.checked_frame <= CHECK_INTERVAL {
            continue;
        }
        let Ok(hull) = spring::get_unit_position(ship.id, false, false) else {
            continue;
        };
        match nearby_ice(hull.x, hull.z) {
            Ok(false) => {}
            Ok(true) => continue,
            Err(error) => {
                log_failed_query(error);
                continue;
            }
        }
        ship.checked_frame = frame;
        if let Some((x, z)) = ship.order
            && distance(x, z, hull.x, hull.z) > NEARBY_SIZE
        {
            order_move(ship.id, x, z);
            continue;
        }
        match find_closest_ice(hull.x, hull.z) {
            Ok(Some((x, z))) => {
                order_move(ship.id, x, z);
                ship.order = Some((x, z));
            }
            Ok(None) => {
                ship.order = None;
                spring::log_info!(LOG_SECTION, "no more ice");
                drills.out_of_ice = true;
                break;
            }
            Err(error) => log_failed_query(error),
        }
    }
}

const LOG_SECTION: &str = "mycube-drills";

fn log_failed_query(error: spring::ApiError) {
    spring::log_once!(
        spring::log_level::WARNING,
        LOG_SECTION,
        "reading the ground heights around a drill ship failed: {error:?}"
    );
}

const NEARBY_SIZE: f32 = 100.0;

fn order_move(unit: i32, x: f32, z: f32) {
    let y = spring::get_ground_height(x, z).unwrap_or(0.0);
    spring::commands::give_order_to_unit(unit, spring::cmd::MOVE, &[x, y, z], 0, 0).then(
        move |result| {
            if let Err(error) = result
                && crate::units::is_alive(unit)
            {
                spring::log_warning!(
                    LOG_SECTION,
                    "move order for drill ship {unit} failed: {error:?}"
                );
            }
        },
    );
}

fn distance(x1: f32, z1: f32, x2: f32, z2: f32) -> f32 {
    let dx = x1 - x2;
    let dz = z1 - z2;
    spring::Float3::new(dx, 0.0, dz).length()
}

fn nearby_ice(x: f32, z: f32) -> spring::Result<bool> {
    let half = NEARBY_SIZE / 2.0;
    let start_x = round_to_square(x - half);
    let start_z = round_to_square(z - half);
    let size = round_to_square(NEARBY_SIZE) as i32;
    let mut points = Vec::new();
    for dx in (0..=size).step_by(SQUARE_SIZE as usize) {
        for dz in (0..=size).step_by(SQUARE_SIZE as usize) {
            points.push([start_x + dx as f32, start_z + dz as f32]);
        }
    }
    Ok(spring::ground_heights(&points)?
        .into_iter()
        .any(|height| height > ICE_HEIGHT))
}

fn find_closest_ice(x: f32, z: f32) -> spring::Result<Option<(f32, f32)>> {
    let (map_x, map_z) = spring::game::map_size_elmos()?;
    let center_x = round_to_square(x) as i32;
    let center_z = round_to_square(z) as i32;
    let on_map = |x: i32, z: i32| (0..=map_x).contains(&x) && (0..=map_z).contains(&z);
    let step = SQUARE_SIZE as usize;
    let mut ring = Vec::new();
    for size in (SQUARE_SIZE..=map_x.max(map_z)).step_by(step) {
        ring.clear();
        for z in [center_z - size, center_z + size] {
            for x in (center_x - size..=center_x + size).step_by(step) {
                ring.push((x, z));
            }
        }
        for x in [center_x - size, center_x + size] {
            for z in (center_z - size..=center_z + size).step_by(step) {
                ring.push((x, z));
            }
        }
        ring.retain(|&(x, z)| on_map(x, z));
        if ring.is_empty() {
            continue;
        }
        let points: Vec<[f32; 2]> = ring.iter().map(|&(x, z)| [x as f32, z as f32]).collect();
        let heights = spring::ground_heights(&points)?;
        for ((x, z), height) in ring.iter().copied().zip(heights) {
            if height > ICE_HEIGHT {
                return Ok(Some((x as f32, z as f32)));
            }
        }
    }
    Ok(None)
}
