use alloc::vec::Vec;

use mycube_shared::unit;
use spring::cus::{Angle, AngularSpeed, Axis, Piece, UnitCtx};
use spring::{ResultExt, UnitId};

pub(super) const SIG_WALK: spring::cus::SignalMask = spring::cus::SignalMask::new(1);
pub(super) const SIG_IDLE: spring::cus::SignalMask = spring::cus::SignalMask::new(2);
pub(super) const SIG_AIM: spring::cus::SignalMask = spring::cus::SignalMask::new(4);

pub(super) type PieceMap<'a> = [(&'a str, Piece)];

pub(super) fn piece_in(map: &PieceMap, name: &str) -> Piece {
    map.iter()
        .find(|(entry, _)| *entry == name)
        .map_or(Piece::INVALID, |(_, piece)| *piece)
}

fn piece_num(piece: Piece) -> i32 {
    piece.0 + 1
}

pub(super) fn piece_positions<const N: usize>(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit: UnitId,
    map: &PieceMap,
    names: [&str; N],
) -> [spring::Float3; N] {
    let mut positions = [spring::Float3::ZERO; N];
    let Ok(root) = spring.units_pieces().get_unit_root_piece(unit.0) else {
        return positions;
    };
    let mut stack = alloc::vec![(root, spring::Float3::ZERO)];
    while let Some((num, parent_offset)) = stack.pop() {
        let Ok(result) = spring.units_pieces().get_unit_piece_info(unit.0, num) else {
            continue;
        };
        if !result.exists {
            continue;
        }
        let info = result.info;
        let offset = spring::Float3::new(
            parent_offset.x + info.offset.x,
            parent_offset.y + info.offset.y,
            parent_offset.z + info.offset.z,
        );
        for (index, name) in names.iter().enumerate() {
            if map
                .iter()
                .any(|(entry, piece)| piece_num(*piece) == num && entry == name)
            {
                positions[index] = offset;
            }
        }
        for child_name in &info.children {
            if let Some((_, child)) = map.iter().find(|(entry, _)| entry == child_name) {
                stack.push((piece_num(*child), offset));
            }
        }
    }
    positions
}

pub(super) fn turn(ctx: &UnitCtx, piece: Piece, axis: Axis, target: f32, speed: f32) {
    if speed == 0.0 {
        ctx.turn_now(piece, axis, Angle::radians(target));
    } else {
        ctx.turn(
            piece,
            axis,
            Angle::radians(target),
            AngularSpeed::radians_per_second(speed),
        );
    }
}

pub(super) fn move_piece(ctx: &UnitCtx, piece: Piece, axis: Axis, target: f32, speed: f32) {
    if speed == 0.0 {
        ctx.move_now(piece, axis, target);
    } else {
        ctx.move_piece(piece, axis, target, speed);
    }
}

pub(super) fn coin_flip(ctx: &UnitCtx) -> bool {
    random_int(ctx, 0, 1) == 1
}

pub(super) fn random_int(ctx: &UnitCtx, lower: i32, upper: i32) -> i32 {
    ctx.with_spring(|spring| spring.synced_random().next_int(lower, upper))
        .expect("synced random int")
}

pub(super) fn random_float(ctx: &UnitCtx) -> f32 {
    ctx.with_spring(|spring| spring.synced_random().next_float())
        .expect("synced random float")
}

pub(super) fn set_unit_geometry(ctx: &UnitCtx, radius: f32, height: f32) {
    ctx.with_spring(|spring| {
        spring
            .unit_control()
            .set_unit_mid_and_aim_pos(
                ctx.unit().0,
                spring::Float3::new(0.0, 30.0, 0.0),
                spring::Float3::new(0.0, 30.0, 0.0),
                true,
            )
            .log_err_once(spring, "setting a unit's mid and aim position failed");
        spring
            .unit_control()
            .set_unit_radius_and_height(ctx.unit().0, radius, height)
            .log_err_once(spring, "setting a unit's radius and height failed");
        Ok(())
    })
    .expect("unit-script engine scope");
}

pub(super) fn all_named_pieces(map: &PieceMap) -> Vec<Piece> {
    map.iter().map(|(_, piece)| *piece).collect()
}

fn group_and_number(name: &str) -> Option<(&str, u32)> {
    let group = name.trim_end_matches(|character: char| character.is_ascii_digit());
    let number = name[group.len()..].parse().ok()?;
    Some((group, number))
}

pub(super) fn numbered_group(map: &PieceMap, group: &str) -> Vec<(u32, Piece)> {
    let mut pieces = map
        .iter()
        .filter_map(|(name, piece)| {
            let (name, number) = group_and_number(name)?;
            (name == group).then_some((number, *piece))
        })
        .collect::<Vec<_>>();
    pieces.sort_by_key(|(number, piece)| (*number, piece.0));
    pieces
}

pub(super) fn matching_pieces(map: &PieceMap, group: &str) -> Vec<Piece> {
    numbered_group(map, group)
        .into_iter()
        .map(|(_, piece)| piece)
        .collect()
}

pub(super) fn grouped_pieces(map: &PieceMap) -> Vec<Piece> {
    let mut pieces = map
        .iter()
        .filter_map(|(name, piece)| {
            let (group, number) = group_and_number(name)?;
            Some(((group, number), *piece))
        })
        .collect::<Vec<_>>();
    pieces.sort_by_key(|(key, _)| *key);
    pieces.into_iter().map(|(_, piece)| piece).collect()
}

pub(super) fn hide_all(ctx: &UnitCtx, pieces: &[Piece]) {
    for piece in pieces {
        ctx.hide(*piece);
    }
}

pub(super) fn show_all(ctx: &UnitCtx, pieces: &[Piece]) {
    for piece in pieces {
        ctx.show(*piece);
    }
}

pub(super) fn reset_all(ctx: &UnitCtx, pieces: &[Piece]) {
    for piece in pieces {
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            ctx.stop_spin(*piece, axis, AngularSpeed::radians_per_second(0.0));
            ctx.turn_now(*piece, axis, Angle::radians(0.0));
            ctx.move_now(*piece, axis, 0.0);
        }
    }
}

pub(super) fn set_scenery_state(ctx: &UnitCtx) {
    ctx.with_spring(|spring| {
        spring
            .unit_control()
            .set_unit_neutral(ctx.unit().0, true)
            .log_err(spring, "making the scenery neutral failed");
        spring
            .rules_params()
            .set_unit_rules_param_f32(
                ctx.unit().0,
                unit::UNTARGETABLE,
                1.0,
                spring::rules_params::los::PUBLIC,
            )
            .log_err(spring, "marking the scenery untargetable failed");
        Ok(())
    })
    .expect("unit-script engine scope");
}

pub(super) fn play_unit_sound(ctx: &UnitCtx, name: &str, volume: f32) {
    ctx.with_spring(|spring| {
        let position = spring
            .units_info()
            .get_unit_position(ctx.unit().0, Default::default())
            .unwrap_or(spring::Float3::ZERO);
        let _ = spring
            .sound()
            .play_sound_file(name, volume, position, spring::Float3::ZERO, 0);
        Ok(())
    })
    .expect("unit-script engine scope");
}
