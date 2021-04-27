use std::fmt;
use std::collections::{
    HashMap,
};
use std::ops::*;

use legion::*;

/// Виды помещений
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum AreaType {
    Living, // жилячейки
    Science, // лаборатории
    Military, // казармы
    Industrial, // технические и производственные помещения. терминалы, распределительные узлы, насосы, чаны, станки.
    Party, // склады, образовательные помещения, детские сады, школы, залы партсобраний
}

impl fmt::Display for AreaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AreaType::Living     => write!(f, "{}", "Жилое"),
            AreaType::Science    => write!(f, "{}", "Научное"),
            AreaType::Military   => write!(f, "{}", "Казармы"),
            AreaType::Industrial => write!(f, "{}", "Производственное"),
            AreaType::Party      => write!(f, "{}", "Партийное"),
        }
    }
}

/// Вместимость помещения (квадратные сантиметры)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AreaCapacity(pub usize);

impl From<AreaCapacity> for usize {
    fn from(val: AreaCapacity) -> usize {
        val.0
    }
}

/// Занятая площадь (квадратные сантиметры)
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AreaOccupied(pub usize);

impl Add for AreaOccupied {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl AddAssign for AreaOccupied {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

/// Свободная площадь (квадратные сантиметры)
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd )]
pub struct AreaFree(pub usize);

/// Метка того, к какой комнате принадлежит эта штука
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BelongsToRoom (pub Entity);

/// Какие комнаты есть и сколько в них места
pub fn rooms_with_space(
    world: &mut World
) -> Vec<(AreaType, AreaFree)> {
    let mut query = <(
        Entity,
        &AreaType,
    )>::query();
    let mut areas = Vec::new();
    for (entity, area_type) in query.iter(world) {
        areas.push((entity.clone(), area_type.clone()));
    };
    let mut result = Vec::new();
    for (entity, area_type) in areas.iter() {
        let fspace = get_room_free_space(world, entity.clone());
        result.push((*area_type, fspace));
    };
    result
}

/// Узнать сколько в комнате осталось места
pub fn get_room_free_space(
    world: &mut World,
    room: Entity,
) -> AreaFree {
    let AreaCapacity (capacity) = world
        .entry(room)
        .unwrap()
        .into_component::<AreaCapacity>()
        .unwrap()
        .clone();
    let mut query = <(
        &BelongsToRoom,
        &AreaOccupied,
    )>::query();
    let mut sum:usize = 0;
    for occupied in query.iter(world)
        .filter(
            |(&BelongsToRoom(entity), _)|
            entity == room
        ).map(|tup|tup.1) {
            let occupied_ = occupied.0;
            sum += occupied_;
        };
    AreaFree(capacity - sum)
}

/// Есть ли у нас комната этого назначения
/// в которую вместится нечто указанного размера
pub fn get_sufficent_room(
    world: &mut World,
    for_: AreaOccupied,
    type_: AreaType,
) -> Option<Entity> {
    let mut areas: HashMap<Entity, AreaFree> = HashMap::new();

    let mut areasq = <(
        Entity,
        &AreaType,
        &AreaCapacity,
    )>::query();
    for (entity, _, capacity) in areasq
        .iter(world)
        .filter(|(_, artype, _)| **artype == type_)
    {
        areas.insert(*entity, AreaFree(capacity.0));
    }

    let mut volumeq = <(
        &BelongsToRoom,
        &AreaOccupied,
    )>::query();

    // Собираем заполненность помещений
    for (room, volume) in volumeq.iter(world) {
        match areas.get_mut(&room.0) {
            Some(free) => free.0 += volume.0,
            None => (),
        }
    };

    let mut areas_vec:Vec<(Entity, AreaFree)> = areas
        .iter()
        .map(|(k,v)| {(*k, *v)})
        .filter(|(_, f)|{f.0 >= for_.0})
        .collect();

    // берем наиболее забитые помещения
    // но в которые тем не менее вместится то что нам надо
    areas_vec
        .sort_by (
            |(_, f1), (_, f2)|
            {(f1.0 as usize).cmp(&(f2.0 as usize))}
        );
    match areas_vec.pop () {
        Some((e, _)) => Some (e),
        None => None,
    }
}
