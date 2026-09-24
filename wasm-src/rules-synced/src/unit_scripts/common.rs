use alloc::vec::Vec;
use mycube_shared::unit;

use spring::UnitId;
use spring::cus::{Angle, AngularSpeed, Axis, Piece, UnitCtx};

pub(super) const SIG_WALK: spring::cus::SignalMask = spring::cus::SignalMask::new(1);
pub(super) const SIG_IDLE: spring::cus::SignalMask = spring::cus::SignalMask::new(2);
pub(super) const SIG_AIM: spring::cus::SignalMask = spring::cus::SignalMask::new(4);

/// Every piece of a unit by model name, in model order (`InitCtx::pieces`).
pub(super) type PieceMap<'a> = [(&'a str, Piece)];

/// A piece by model name, or `Piece::INVALID`.
pub(super) fn piece_in(map: &PieceMap, name: &str) -> Piece {
    map.iter()
        .find(|(entry, _)| *entry == name)
        .map_or(Piece::INVALID, |(_, piece)| *piece)
}

/// The engine's piece number (1-based) of a script piece.
fn piece_num(piece: Piece) -> i32 {
    piece.0 + 1
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
    while let Some((num, parent_offset)) = stack.pop() {
        let Ok(result) = spring::get_unit_piece_info(unit.0, num) else {
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

/// Lua's `math.random(0, 1) == 1`: a synced coin flip.
pub(super) fn coin_flip() -> bool {
    spring::random::synced::int(0, 1) == 1
}

pub(super) fn all_named_pieces(map: &PieceMap) -> Vec<Piece> {
    map.iter().map(|(_, piece)| *piece).collect()
}

/// The group and number of a piece named like `Build3` or `Deco005`, the way
/// Lua's `getPieceTableByNameGroups` splits names: the trailing digits are
/// the number, the rest is the group. Pieces without a number are in none.
fn group_and_number(name: &str) -> Option<(&str, u32)> {
    let group = name.trim_end_matches(|character: char| character.is_ascii_digit());
    let number = name[group.len()..].parse().ok()?;
    Some((group, number))
}

/// `TableOfPieceGroups[group]`: the pieces of a group with their numbers,
/// in number order (Lua keys the table by number).
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

/// The pieces of a group, in number order.
pub(super) fn matching_pieces(map: &PieceMap, group: &str) -> Vec<Piece> {
    numbered_group(map, group)
        .into_iter()
        .map(|(_, piece)| piece)
        .collect()
}

/// Every piece that belongs to a numbered group, i.e. everything in Lua's
/// `TableOfPieceGroups`.
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

pub(super) fn set_scenery_state(unit: UnitId) {
    let _ = spring::set_unit_neutral(unit.0, true);
    let _ = spring::set_unit_rules_param_f32(
        unit.0,
        unit::UNTARGETABLE,
        1.0,
        spring::rules_params::los::PUBLIC,
    );
}

pub(super) fn play_unit_sound(unit: UnitId, name: &str, volume: f32) {
    let position = spring::get_unit_position(unit.0, false, false).unwrap_or(spring::Float3::ZERO);
    let _ = spring::sound::play_sound_file(name, volume, position, spring::Float3::ZERO, 0);
}
