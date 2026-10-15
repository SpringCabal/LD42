use alloc::vec::Vec;

use spring::cus::{Angle, AngularSpeed, Axis, Piece, SfxFlags, UnitCtx};

use super::common::{
    PieceMap, all_named_pieces, coin_flip, grouped_pieces, hide_all, matching_pieces,
    numbered_group, piece_in, show_all,
};

pub struct CoalburnerScript {
    all: Vec<Piece>,
    heaters: Vec<Piece>,
    firecycle: Vec<Piece>,
    inards: Piece,
}

impl CoalburnerScript {
    pub(super) fn new(map: &PieceMap) -> Self {
        Self {
            all: all_named_pieces(map),
            heaters: matching_pieces(map, "Heater"),
            firecycle: matching_pieces(map, "firecycle"),
            inards: piece_in(map, "Inards"),
        }
    }

    pub(super) fn create(&mut self, ctx: &UnitCtx) {
        hide_all(ctx, &self.all);
        self.set_heat(ctx, 0.0);
    }

    pub(super) fn set_heat(&mut self, ctx: &UnitCtx, factor: f32) {
        hide_all(ctx, &self.heaters);
        hide_all(ctx, &self.firecycle);
        ctx.hide(self.inards);
        let factor = factor.clamp(0.0, 1.0);
        let index = ((factor * 4.0 + 1.0) as usize).clamp(1, 4);
        if index == 1 {
            ctx.show(self.inards);
        }
        if let Some(piece) = self.heaters.get(index - 1) {
            ctx.show(*piece);
        }
        if factor > 0.25 {
            show_all(ctx, &self.firecycle);
            for (index, piece) in self.firecycle.iter().enumerate() {
                let speed = if index == 0 { -150.0 } else { 150.0 };
                ctx.spin(
                    *piece,
                    Axis::X,
                    AngularSpeed::degrees_per_second(speed),
                    AngularSpeed::radians_per_second(0.0),
                );
            }
        }
    }
}

struct IgluBlock {
    piece: Piece,
    xy_deco: Vec<Piece>,
    y_deco: Vec<Piece>,
}

pub struct IgluScript {
    all: Vec<Piece>,
    grouped: Vec<Piece>,
    blocks: Vec<IgluBlock>,
}

impl IgluScript {
    pub(super) fn new(map: &PieceMap) -> Self {
        let blocks = numbered_group(map, "Build")
            .into_iter()
            .map(|(number, piece)| IgluBlock {
                piece,
                xy_deco: matching_pieces(map, &alloc::format!("XY{number}Deco")),
                y_deco: matching_pieces(map, &alloc::format!("Y{number}Deco")),
            })
            .collect();
        Self {
            all: all_named_pieces(map),
            grouped: grouped_pieces(map),
            blocks,
        }
    }

    pub(super) fn create(&mut self, ctx: &UnitCtx) {
        hide_all(ctx, &self.all);
        if let Some(block) = self.blocks.first() {
            ctx.show(block.piece);
        }
        for block in &self.blocks {
            if !coin_flip() {
                continue;
            }
            ctx.show(block.piece);
            turn_degrees(ctx, block.piece, Axis::Y, -360, 360);
            for piece in block.xy_deco.iter().copied() {
                if coin_flip() {
                    turn_degrees(ctx, piece, Axis::Y, -360, 360);
                    turn_degrees(ctx, piece, Axis::X, -80, 15);
                    ctx.show(piece);
                }
            }
            for piece in block.y_deco.iter().copied() {
                if coin_flip() {
                    turn_degrees(ctx, piece, Axis::Y, -360, 360);
                    ctx.show(piece);
                }
            }
        }
    }

    pub(super) fn killed(&mut self, ctx: &UnitCtx) {
        for piece in self.grouped.iter().copied() {
            let effect = if coin_flip() {
                SfxFlags::FALL
            } else {
                SfxFlags::SHATTER
            };
            ctx.explode(piece, SfxFlags(effect.0 | SfxFlags::NO_HEATCLOUD.0));
        }
    }
}

fn turn_degrees(ctx: &UnitCtx, piece: Piece, axis: Axis, lower: i32, upper: i32) {
    let angle = spring::random::synced::int(lower, upper) as f32;
    ctx.turn_now(piece, axis, Angle::degrees(angle));
}
