// Специальный модуль для топ левел запросов, которые не относятся к какому то конкретному модулю, а захватывают типы из нескольких

use std::collections::HashMap;
use legion::*;
use crate::{
    area::*,
    core::*,
    people::*,
    production::*
};

/// кто какое место занимает
pub fn who_take_place(
    world: &mut World,
) -> HashMap<Entity, Vec<(Profession, Tier, AreaOccupied)>> {
    let mut result = HashMap::new ();
    let mut query = <(&BelongsToRoom, &Profession, &Tier, &AreaOccupied)>::query();
    for (&BelongsToRoom(entity), prof, tier, occupied) in query.iter(world) {
        result
            .entry(entity)
            .and_modify(
                |v: &mut Vec<(Profession, Tier, AreaOccupied)>| {
                    v.push((
                        *prof,
                        *tier,
                        *occupied,
                    ))
                }
            )
            .or_insert(
                vec![
                    (
                        *prof,
                        *tier,
                        *occupied,
                    )
                ]
            );
    };
    result
}

/// Что какое место занимает
pub fn what_take_place(
    world: &mut World,
) -> HashMap <Entity, Vec<(Stationary, AreaOccupied, TaskStatus)>> {
    let mut result = HashMap::new();
    let mut query = <(
        &BelongsToRoom,
        &Stationary,
        &AreaOccupied,
        &TaskStatus,
    )>::query();

    for (
        &BelongsToRoom(entity),
        stationary,
        occupied,
        status,
    ) in query.iter(world) {
        result
            .entry(entity)
            .and_modify(
                |v: &mut Vec<(Stationary, AreaOccupied, TaskStatus)>| {
                    v.push ((
                        *stationary,
                        *occupied,
                        status.clone(),
                    ));
                }
            ).or_insert(vec![(
                *stationary,
                *occupied,
                status.clone(),
            )]);
    };
    result
}
