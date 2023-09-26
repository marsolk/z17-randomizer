use crate::legacy::path::Path;
use crate::model::check::Check;
use crate::model::filler_item::Goal;
use crate::model::location::Location::{self, *};
use crate::model::location_node::LocationNode;
use crate::model::logic::Logic;
use crate::regions;
use crate::world::{check, edge, goal, location, old_check, old_path};
use crate::LocationInfo;

use std::collections::HashMap;

pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            SwampPalaceOutside,
            location(
                "Swamp Palace Outside",
                vec![],
                vec![
                    old_path(
                        LoruleCastleField,
                        Some(|p| p.has_hookshot() || p.has_flippers() || p.has_bomb_flower()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    edge!(SwampPalaceAntechamber),
                ],
            ),
        ),
        (
            SwampPalaceAntechamber,
            location(
                "Swamp Palace Antechamber",
                vec![],
                vec![
                    edge!(SwampPalaceOutside),
                    old_path(
                        SwampPalaceFoyer,
                        Some(|p| p.has_bomb_flower()),
                        None,
                        None,
                        Some(|p| {
                            p.not_nice_mode()
                                && p.can_merge()
                                && p.has_ice_rod()
                                && p.has_flippers()
                                && (p.has_sword()
                                    || p.has_tornado_rod()
                                    || p.has_net()
                                    || p.has_bombs())
                        }),
                        None,
                    ),
                ],
            ),
        ),
        (
            SwampPalaceFoyer,
            location(
                "Swamp Palace Foyer",
                vec![],
                vec![
                    edge!(SwampPalaceAntechamber),
                    old_path(
                        SwampPalaceMain,
                        Some(|p| p.has_flippers() && p.has_hookshot()),
                        None,
                        None, // what a cruel game
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SwampPalaceMain,
            location(
                "Swamp Palace",
                vec![
                    check!("[SP] (B1) Center", regions::dungeons::swamp::palace::SUBREGION),
                    check!("[SP] (B1) Waterfall Room", regions::dungeons::swamp::palace::SUBREGION),
                    check!(
                        "[SP] (B1) Raft Room (Pillar)",
                        regions::dungeons::swamp::palace::SUBREGION
                    ),
                    check!(
                        "[SP] (B1) Raft Room (Right)",
                        regions::dungeons::swamp::palace::SUBREGION
                    ),
                    check!(
                        "[SP] (B1) Raft Room (Left)",
                        regions::dungeons::swamp::palace::SUBREGION
                    ),
                    check!("[SP] (B1) Gyorm", regions::dungeons::swamp::palace::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[SP] (B1) Big Chest (Secret)",
                            regions::dungeons::swamp::palace::SUBREGION,
                        ),
                        Some(|p| {
                            p.has_swamp_keys(2)
                                && p.can_merge()
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        Some(|p| {
                            p.has_swamp_keys(2)
                                && p.has_bow()
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        Some(|p| p.has_swamp_keys(2) && p.has_boots()),
                        Some(|p| p.has_swamp_keys(2) && p.not_nice_mode() && p.has_ice_rod()),
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[SP] (1F) West Room",
                            regions::dungeons::swamp::palace::SUBREGION,
                        ),
                        Some(|p| {
                            p.has_swamp_keys(2)
                                && p.can_merge()
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        None,
                        None,
                        Some(|p| p.not_nice_mode() && p.has_ice_rod()),
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[SP] (1F) East Room",
                            regions::dungeons::swamp::palace::SUBREGION,
                        ),
                        Some(|p| {
                            p.has_swamp_keys(2)
                                && p.can_merge()
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        None,
                        None,
                        Some(|p| p.not_nice_mode() && p.has_ice_rod()),
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[SP] (1F) Water Puzzle",
                            regions::dungeons::swamp::palace::SUBREGION,
                        ),
                        Some(|p| {
                            p.has_swamp_keys(2)
                                && p.can_merge()
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        None,
                        None,
                        Some(|p| p.not_nice_mode() && p.can_merge() && p.has_ice_rod()),
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[SP] (1F) Big Chest (Fire)",
                            regions::dungeons::swamp::palace::SUBREGION,
                        ),
                        Some(|p| {
                            p.can_merge()
                                && (p.progression_enemies() || p.has_bombs() || p.has_hammer())
                                && (p.has_swamp_keys(4)
                                    || (p.has_swamp_keys(2)
                                        && (p.has_tornado_rod() || p.has_ice_rod())))
                        }),
                        Some(|p| {
                            p.can_merge()
                                && (p.progression_enemies() || p.has_bombs() || p.has_hammer())
                                && p.has_swamp_keys(2)
                        }),
                        Some(|p| p.has_boots()),
                        Some(|p| p.not_nice_mode() && p.has_ice_rod()),
                        None,
                    ),
                ],
                vec![old_path(
                    SwampPalacePostBoss,
                    Some(|p| {
                        p.can_merge()
                            && (p.progression_enemies() || p.has_bombs() || p.has_hammer())
                            && p.has_swamp_keys(4)
                            && p.has_swamp_big_key()
                            && p.can_defeat_arrgus()
                    }),
                    None,
                    None,
                    Some(|p| {
                        p.not_nice_mode()
                            && p.has_ice_rod()
                            && (p.has_swamp_big_key() || p.has_tornado_rod())
                    }),
                    None,
                )],
            ),
        ),
        (
            SwampPalacePostBoss,
            location(
                "Swamp Palace Post Boss",
                vec![
                    check!("[SP] Arrghus", regions::dungeons::swamp::palace::SUBREGION),
                    check!("Swamp Palace Prize", regions::dungeons::swamp::palace::SUBREGION),
                    goal!("Arrghus", Goal::Arrghus),
                ],
                vec![],
            ),
        ),
    ])
}