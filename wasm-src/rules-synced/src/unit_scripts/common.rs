use alloc::vec::Vec;

use spring::UnitId;
use spring::cus::{Angle, AngularSpeed, Axis, Piece, PieceResolver, UnitCtx};

pub(super) const SIG_WALK: spring::cus::SignalMask = spring::cus::SignalMask::new(1);
pub(super) const SIG_IDLE: spring::cus::SignalMask = spring::cus::SignalMask::new(2);
pub(super) const SIG_AIM: spring::cus::SignalMask = spring::cus::SignalMask::new(4);
pub(super) const SIG_SHIP: spring::cus::SignalMask = spring::cus::SignalMask::new(8);

pub(super) struct EnginePieces {
    pub(super) map: Vec<spring::owned::units_pieces::PieceMapEntry>,
    pub(super) root: Piece,
}

impl PieceResolver for EnginePieces {
    fn piece(&self, name: &str) -> Piece {
        if name == "__mycube_model_root" {
            return self.root;
        }
        self.map
            .iter()
            .find(|entry| entry.name == name)
            .map_or(Piece::INVALID, |entry| Piece(entry.piece_num - 1))
    }
}

pub(super) fn piece_by_name(unit: UnitId, name: &str) -> Piece {
    spring::get_unit_piece_map(unit.0)
        .ok()
        .and_then(|pieces| {
            pieces
                .into_iter()
                .find(|piece| piece.name == name)
                .map(|piece| Piece(piece.piece_num - 1))
        })
        .unwrap_or(Piece::INVALID)
}

pub(super) fn piece_position(unit: UnitId, name: &str) -> spring::Float3 {
    let map = match spring::get_unit_piece_map(unit.0) {
        Ok(map) => map,
        Err(_) => return spring::Float3::ZERO,
    };
    let target = match map.iter().find(|entry| entry.name == name) {
        Some(entry) => entry.piece_num,
        None => return spring::Float3::ZERO,
    };
    let root = match spring::get_unit_root_piece(unit.0) {
        Ok(root) => root,
        Err(_) => return spring::Float3::ZERO,
    };

    fn add(left: spring::Float3, right: spring::Float3) -> spring::Float3 {
        spring::Float3 {
            x: left.x + right.x,
            y: left.y + right.y,
            z: left.z + right.z,
        }
    }

    fn find_offset(
        unit: UnitId,
        piece_num: i32,
        target: i32,
        parent_offset: spring::Float3,
        map: &[spring::owned::units_pieces::PieceMapEntry],
    ) -> Option<spring::Float3> {
        let result = spring::get_unit_piece_info(unit.0, piece_num).ok()?;
        if !result.exists {
            return None;
        }
        let offset = add(parent_offset, result.info.offset);
        if piece_num == target {
            return Some(offset);
        }
        for child_name in result.info.children {
            let child_num = map
                .iter()
                .find(|entry| entry.name == child_name)
                .map(|entry| entry.piece_num)?;
            if let Some(child_offset) = find_offset(unit, child_num, target, offset, map) {
                return Some(child_offset);
            }
        }
        None
    }

    find_offset(unit, root, target, spring::Float3::ZERO, &map).unwrap_or(spring::Float3::ZERO)
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

pub(super) fn explode_flags(flags: i32) -> spring::cus::SfxFlags {
    spring::cus::SfxFlags(flags)
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

pub(super) fn all_named_pieces(map: &[spring::owned::units_pieces::PieceMapEntry]) -> Vec<Piece> {
    map.iter().map(|entry| Piece(entry.piece_num - 1)).collect()
}

pub(super) fn matching_pieces(
    map: &[spring::owned::units_pieces::PieceMapEntry],
    prefix: &str,
) -> Vec<Piece> {
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
    let _ = spring::set_unit_rules_param_f32(unit.0, "untargetable", 1.0, crate::state::PUBLIC);
}

pub(super) fn play_unit_sound(unit: UnitId, name: &str, volume: f32) {
    let position = spring::get_unit_position(unit.0, false, false).unwrap_or(spring::Float3::ZERO);
    let _ = spring::sound::play_sound_file(name, volume, position, spring::Float3::ZERO, 0);
}
