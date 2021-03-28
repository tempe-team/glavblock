use std::cmp::min;
use std::ops::*;
use std::collections::HashMap;

use legion::*;

use crate::area::*;
use crate::core::*;
use crate::resources::*;

/// Вещественные единицы (количество ресурса)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RealUnits (pub usize);

impl SubAssign for RealUnits {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl AddAssign for RealUnits {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for RealUnits {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

/// какое простанство занимает 1 единица этого ресурса
pub fn get_piece_size (
    resource: Resource,
) -> AreaOccupied {
    match resource {
        Resource::BioRaw           => AreaOccupied(10),
        Resource::ScrapT1          => AreaOccupied(100),
        Resource::ScrapT2          => AreaOccupied(100),
        Resource::ScrapT3          => AreaOccupied(100),
        Resource::Concrete         => AreaOccupied(100),
        Resource::Slime            => AreaOccupied(10),
        Resource::ComponentT1      => AreaOccupied(10),
        Resource::ComponentT2      => AreaOccupied(10),
        Resource::ComponentT3      => AreaOccupied(10),
        Resource::ReagentT1        => AreaOccupied(10),
        Resource::ReagentT2        => AreaOccupied(10),
        Resource::ReagentT3        => AreaOccupied(10),
        Resource::Polymer          => AreaOccupied(50),
        Resource::Concentrat       => AreaOccupied(1),
    }
}

/// Куча ресурсов занимает столько места. Сколько там единиц ресурса
pub fn volume2real (
    resource: Resource,
    volume: AreaOccupied
) -> RealUnits {
    let piece_size = get_piece_size (resource);
    RealUnits((volume.0 / piece_size.0) as usize)
}

/// Партию ресурсов можно распределить по нескольким местам хранения
pub fn get_rooms_for_divisible_load(
    world: &mut World,
    resource: Resource,
    amount: RealUnits,
) -> HashMap<Entity, AreaFree> {
    let mut amount_ = amount.0;
    let mut rooms =
        HashMap::new();
    let mut rooms_query =
        <(Entity, &AreaType, &AreaCapacity)>::query();
    for (e, _, capacity) in rooms_query
        .iter(world)
        .filter(
            |(_, type_,  _)|
            **type_ == AreaType::Party
        ) {
            rooms.insert(*e, AreaFree(capacity.0));
        };
    let mut content_query =
        <(&BelongsToRoom, &AreaOccupied)>::query();
    for (BelongsToRoom(room), occupied) in content_query.iter(world) {
        rooms.entry(*room).and_modify (|free| { free.0 -= occupied.0});
    };
    rooms
}

/// Положить ресурс на хранение.
/// Возвращает количество невместившегося ресурса.
pub fn put_resource(
    world: &mut World,
    resource: Resource,
    amount: RealUnits,
) -> RealUnits {
    let piece_size = get_piece_size(resource);
    let mut amount_ = amount.clone();
    let rooms =
        get_rooms_for_divisible_load(
            world,
            resource,
            amount,
        );
    for (room, area) in rooms.iter () {
        let required_space =
            piece_size.0 * amount_.0;
        let to_put_here = min(area.0 as usize, required_space as usize);
        amount_.0 -= (to_put_here / piece_size.0) as usize;
        world.push(
            (
                resource,
                BelongsToRoom(room.clone()),
                AreaOccupied(to_put_here),
            )
        );
        if amount.0 <= 0 { break };
    };
    amount_
}

/// Изъять ресурс, освободить пространство.
pub fn writeoff (
    world: &mut World,
    resource: Resource,
    amount: RealUnits,
) -> RealUnits {
    let mut writeoff_query = <(
        Entity,
        &Resource,
        &mut AreaOccupied,
    )>::query();
    let piece_size = get_piece_size(resource);
    let mut amount_ = amount.clone();
    let mut empty_containers = Vec::new();
    for (entity, _, occupied) in
        writeoff_query
        .iter_mut(world)
        .filter(|(_, res, _)| {**res == resource}) {
            let to_get_from_here = min(
                amount_.0 * piece_size.0,
                occupied.0,
            );
            occupied.0 -= to_get_from_here;
            amount_.0 -= to_get_from_here / piece_size.0;
            if occupied.0 <= 0 {
                empty_containers.push(*entity);
            };
        };
    for entity in empty_containers.iter() {
        world.remove (*entity);
    };
    amount_
}

/// сколько у нас на складах этого ресурса?
pub fn how_much_we_have (
    world: &mut World,
    resource: Resource,
) -> RealUnits {
    let mut deposit_query = <(
        &Resource,
        &AreaOccupied
    )>::query();
    let deposit_volume = deposit_query
        .iter(world)
        .filter(|(res, _)| **res == resource)
        .map(|(_, occ)| occ)
        .fold(AreaOccupied(0), |a, b| a + *b);
    volume2real(resource, deposit_volume)
}

/// сколько у нас вообще чего в наличии
pub fn what_we_have(
    world: &mut World,
) -> HashMap<Resource, RealUnits> {
    let mut result: HashMap<Resource, RealUnits> = HashMap::new();
    let mut deposit_query = <(
        &Resource,
        &AreaOccupied
    )>::query();
    for (res, vol) in deposit_query.iter (world) {
        let vol_: &mut RealUnits = result
            .entry(*res)
            .or_insert(RealUnits(0));
        *vol_ += volume2real(*res, *vol);
    };
    result
}

/// Есть ли у нас вот столько разных ресурсов
pub fn enough_resources(
    world: &mut World,
    required: HashMap<Resource, RealUnits>,
) -> bool {
    let mut result = true;
    for (res, amount) in required.iter() {
        let deposit = how_much_we_have(world, *res);
        if deposit < *amount {
            result = false;
            break;
        }
    }
    result
}

/// Списать ресурсы пачкой.
pub fn writeoff_bunch (
    world: &mut World,
    bunch: HashMap<Resource, RealUnits>
) -> Result<(),SamosborError> {
    if enough_resources(world, bunch.clone()) {
        for (res, amount) in bunch.iter() {
            let _ = writeoff(world, *res, *amount);
        }
        Ok (())
    } else {
        Err(SamosborError::NotEnoughResources)
    }
}
