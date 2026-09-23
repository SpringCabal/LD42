use alloc::vec::Vec;

use spring::UnitId;
use spring::cus::{Angle, AngularSpeed, Axis, Piece, UnitCtx};

pub(super) const SIG_WALK: spring::cus::SignalMask = spring::cus::SignalMask::new(1);
pub(super) const SIG_IDLE: spring::cus::SignalMask = spring::cus::SignalMask::new(2);
pub(super) const SIG_AIM: spring::cus::SignalMask = spring::cus::SignalMask::new(4);
pub(super) const SIG_SHIP: spring::cus::SignalMask = spring::cus::SignalMask::new(8);

pub(super) type PieceMap = [spring::owned::units_pieces::PieceMapEntry];

/// A piece by model name, or `Piece::INVALID`.
pub(super) fn piece_in(map: &PieceMap, name: &str) -> Piece {
    map.iter()
        .find(|entry| entry.name == name)
        .map_or(Piece::INVALID, |entry| Piece(entry.piece_num - 1))
}

/// The rest positions (model-space offsets) of the named pieces, from one
/// walk of the piece tree. Missing pieces get `ZERO`.
pub(super) fn piece_positions<const N: usize>(
    unit: UnitId,
    map: &PieceMap,
    names: [&str; N],
) -> [spring::Float3; N] {
    let mut positions = [spring::Float3::ZERO; N];
    let Ok(root) = spring::get_unit_root_piece(unit.0) else {
        return positions;
    };
    let mut stack = alloc::vec![(root, spring::Float3::ZERO)];
    while let Some((piece_num, parent_offset)) = stack.pop() {
        let Ok(result) = spring::get_unit_piece_info(unit.0, piece_num) else {
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
                .any(|entry| entry.piece_num == piece_num && entry.name == *name)
            {
                positions[index] = offset;
            }
        }
        for child_name in &info.children {
            if let Some(child) = map.iter().find(|entry| entry.name == *child_name) {
                stack.push((child.piece_num, offset));
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

pub(super) fn random_next(state: &mut u32) -> u32 {
    // The old Lua helpers use Spring's synced random stream.  A per-script
    // deterministic stream gives the same important property for CUS tasks:
    // animation choices do not depend on host-side timing.
    *state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    *state
}

pub(super) fn random_unit(state: &mut u32) -> f32 {
    (random_next(state) as f32) / (u32::MAX as f32)
}

pub(super) fn all_named_pieces(map: &PieceMap) -> Vec<Piece> {
    map.iter().map(|entry| Piece(entry.piece_num - 1)).collect()
}

pub(super) fn matching_pieces(map: &PieceMap, prefix: &str) -> Vec<Piece> {
    let mut matches = map
        .iter()
        .filter(|entry| entry.name.starts_with(prefix))
        .map(|entry| {
            let suffix = &entry.name[prefix.len()..];
            let number_end = suffix
                .char_indices()
                .find(|(_, character)| !character.is_ascii_digit())
                .map_or(suffix.len(), |(index, _)| index);
            let number = suffix[..number_end].parse::<u32>().unwrap_or(u32::MAX);
            (number, entry.piece_num, Piece(entry.piece_num - 1))
        })
        .collect::<Vec<_>>();
    // Lua's getPieceTableByNameGroups stores numbered pieces at their numeric
    // indexes.  The engine map itself is hierarchical/model order, which is
    // not necessarily the same order (the drillship is a notable example).
    matches.sort_by_key(|(number, piece_num, _)| (*number, *piece_num));
    matches.into_iter().map(|(_, _, piece)| piece).collect()
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

pub(super) fn set_scenery_state(unit: UnitId) {
    let _ = spring::set_unit_neutral(unit.0, true);
    let _ = spring::set_unit_rules_param_f32(
        unit.0,
        "untargetable",
        1.0,
        spring::rules_params::los::PUBLIC,
    );
}

pub(super) fn play_unit_sound(unit: UnitId, name: &str, volume: f32) {
    let position = spring::get_unit_position(unit.0, false, false).unwrap_or(spring::Float3::ZERO);
    let _ = spring::sound::play_sound_file(name, volume, position, spring::Float3::ZERO, 0);
}
