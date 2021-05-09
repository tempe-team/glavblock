use legion::*;
use std::{
    fmt,
    hash::Hash,
    ops::*,
};
use crate::{
    area::*,
    core::*,
    people::*,
    resources::*,
    storage::*
};

use std::collections::{
    HashMap,
    HashSet,
    hash_set::Difference,
    hash_map::RandomState,
};

/// Приоритет задачи
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority(pub usize);

/// Метка того, к какому стационарному объекту принадлежит эта штука
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BelongsToStationary (pub Entity);

/// Стационарные объекты
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Stationary {
    None, // Отсутствие постройки. Заглушка для обозначения того,
    // что некоторые производственные задачи не требуют
    // стационарного оборудования

    // Производство и хранение
    BenchToolT1, // верстак
    BenchToolT2, // токарно-фрезерный
    BenchToolT3, // электроника, электротехника, 3d печать..
    FormatFurnace, // Печь-формовщик. Переплавка металлолома в пригодные материалы. Температурная обработка. Формовка плавких материалов в лист, прокат, трубу и прочее. Вулканизация. Изготовление концентрата.
    LabT1, // Абстрактное научное оборудование.
    LabT2, // Абстрактное научное оборудование. Крутое.
    LabT3, // Абстрактное научное оборудование. Супер крутое.
    Barrel, // Чаны, в которых проходят химические реакции. Используются в комбинации с хим, биолабораторией или печью. Забирают некое сырье, некий реагент и через какое-то время отдают другое сырье или продукт.

    // Инфраструктура
    NeuroTerminal, // Терминал для связи с нейронетом. ЭВМ.
}

impl fmt::Display for Stationary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stationary::None => write!(f, "оборудования не требуется"),
            Stationary::BenchToolT1 => write!(f, "{}", "Верстак"),
            Stationary::BenchToolT2 => write!(f, "{}", "Токарно-фрезерный станок"),
            Stationary::BenchToolT3 => write!(f, "{}", "Молекулярный принтер"),
            Stationary::FormatFurnace => write!(f, "{}", "Гравитационная печь"),
            Stationary::LabT1 => write!(f, "{}", "Лаборатория"),
            Stationary::LabT2 => write!(f, "{}", "Продвинутая лаборатория"),
            Stationary::LabT3 => write!(f, "{}", "Супер лаборатория"),
            Stationary::Barrel => write!(f, "{}", "Чан"),
            Stationary::NeuroTerminal => write!(f, "{}", "Нейротерминал"),
        }
    }
}

/// FIXME: надо генерить список ресурсов напрямую из энума.
/// Хорошо если бы это появилось прям в расте, использовать sturm не хочется.
#[allow(dead_code)]
pub fn all_stationaries () -> Vec<Stationary> {
    vec![
        Stationary::BenchToolT1,
        Stationary::BenchToolT2,
        Stationary::BenchToolT3,
        Stationary::FormatFurnace,
        Stationary::LabT1,
        Stationary::LabT2,
        Stationary::LabT3,
        Stationary::Barrel,
        Stationary::NeuroTerminal,
    ]
}

/// Гермкомплект. Инфраструктура конкертного помещения.
/// Т1 - Жилячейка, Т2 - Цех/Казарма/Лаборатория/Склад, T3 - Гигацех, Суперзавод итд
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Germ {
    GermT1,
    GermT2,
    GermT3,
}

/// Прогресс постройки
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TaskProgress {
    pub bp_required: BuildPower, // сколько всего запланировано билдавера влить
    pub bp_invested: BuildPower, // сколько билдпавера влито
    pub who_should_finish: Vec<(Profession, Tier, Stationary, BuildPower)>, // Какие спецы на каком оборудовании должы закончить
}

impl fmt::Display for TaskProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}%",
            ((self.bp_invested.0 as f32 / self.bp_required.0 as f32) * 100.) as usize
        )
    }
}

/// Сколько единиц площади занимает стационарный объект
pub fn stationary_size (
    stationary: Stationary,
) -> AreaOccupied {
    match stationary  {
        Stationary::None => AreaOccupied(0),
        Stationary::BenchToolT1 => AreaOccupied(2000),
        Stationary::BenchToolT2 => AreaOccupied(2500),
        Stationary::BenchToolT3 => AreaOccupied(5000),
        Stationary::FormatFurnace => AreaOccupied(5000),
        Stationary::LabT1 => AreaOccupied(2000),
        Stationary::LabT2 => AreaOccupied(4000),
        Stationary::LabT3 => AreaOccupied(6000),
        Stationary::Barrel => AreaOccupied(1500),
        Stationary::NeuroTerminal => AreaOccupied(500),
    }
}

/// Поставить герму + обустроить помещение
pub fn install_germ(
    world: &mut World,
    germ: Germ,
    purpose: AreaType,
) -> Entity {
    world.push((
        germ,
        TaskPriority(0),
        TaskStatus::Constructing,
        task_meta2progress(germ_requirements(germ)),
        purpose,
        germ_capacity(germ),
    ))
}

/// Вместимость гермы
fn germ_capacity(germ: Germ) -> AreaCapacity {
    match germ {
        Germ::GermT1 => AreaCapacity(3000),
        Germ::GermT2 => AreaCapacity(15000),
        Germ::GermT3 => AreaCapacity(50000),
    }
}

/// Трудочасы
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct BuildPower(pub usize);

impl Add for BuildPower {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl AddAssign for BuildPower {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl SubAssign for BuildPower {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

/// Эффективность камрада
pub fn tier2comrad_buildpower(
    tier: Tier
) -> BuildPower {
    match tier {
        Tier::NoTier => unreachable!(),
        Tier::T1 => BuildPower(10),
        Tier::T2 => BuildPower(20),
        Tier::T3 => BuildPower(40),
    }
}

/// В ситуации, когда работнику высокого тира нечего делать
/// он может выполнять работу нижних тиров.
/// T2 работник может делать T1 задачи
/// лучше T1 работника.
/// Эта функция отвечает насколько конкретно лучше.
pub fn buildpower_downgrage_coef(
    worker_tier: Tier,
    target_tier: Tier,
    bp: BuildPower,
) -> BuildPower {
    match (worker_tier, target_tier) {
        (Tier::T1, Tier::T1) => bp,
        (Tier::T2, Tier::T2) => bp,
        (Tier::T3, Tier::T3) => bp,
        (Tier::T2, Tier::T1) => BuildPower(bp.0 * 2),
        (Tier::T3, Tier::T2) => BuildPower(bp.0 * 2),
        (Tier::T3, Tier::T1) => BuildPower(bp.0 * 4),
        _ => BuildPower(0),
    }
}

/// Сколько работы можно произвести на данном оборудовании
pub fn stationary_build_power(
    stationary: Stationary,
) -> BuildPower{
    match stationary {
        Stationary::None => BuildPower(usize::MAX),
        Stationary::BenchToolT1 => BuildPower(10),
        Stationary::BenchToolT2 => BuildPower(20),
        Stationary::BenchToolT3 => BuildPower(40),
        Stationary::FormatFurnace => BuildPower(10),
        Stationary::LabT1 => BuildPower(10),
        Stationary::LabT2 => BuildPower(10),
        Stationary::LabT3 => BuildPower(10),
        Stationary::Barrel => BuildPower(10),
        Stationary::NeuroTerminal => BuildPower(10),
    }
}

/// Что нужно по ресурсам чтобы поставить эту стационарку
pub fn stationary_required_resources (
    stationary: Stationary,
) -> HashMap<Resource, RealUnits> {
    match stationary {
        Stationary::None => HashMap::new(),
        Stationary::BenchToolT1 => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::BenchToolT2 => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::BenchToolT3 => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::FormatFurnace => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::LabT1 => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::LabT2 => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::LabT3 => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::Barrel => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::NeuroTerminal => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
    }
}

/// Метаданные по рабочей задаче
/// Где-то рядом с этой рабочей задачей в ECS лежит штука
/// которая собственно делается
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct TaskMeta {
    pub prof: Profession,
    pub tier: Tier, // Тир исполнителя
    pub bp: BuildPower,
    pub stationary: Stationary, // на каком оборудовании надо выполнять работу
}

/// Преобразовать метовое описание того, как сделать какую то задачу
/// в статус её выполнения.
pub fn task_meta2progress(
    meta: HashSet<TaskMeta>
) -> TaskProgress {
    let bp_required = meta
        .iter()
        .fold(
            BuildPower(0),
            |acc, x| acc + x.bp
        );
    let bp_invested = BuildPower (0);
    let who_should_finish = meta
        .iter()
        .fold(
            Vec::new(),
            |mut acc, m| {
                acc.push((m.prof, m.tier, m.stationary, m.bp));
                acc
            }
        );
    TaskProgress {
        bp_required,
        bp_invested,
        who_should_finish,
    }
}

/// Рендеринг конкретной задачи в формат для отображения
pub fn display_task_meta(
    tm: &TaskMeta
) -> String {
    let rank: &str = match tm.tier {
        Tier::T1     => "1 разряда",
        Tier::T2     => "2 разряда",
        Tier::T3     => "3 разряда",
        Tier::NoTier => "без разряда",
    };
    format!(
        "{} трудочасов, {} {}, {}",
        tm.bp.0,
        tm.prof,
        rank,
        tm.stationary
    )
}

/// Приоритет задачи
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct TaskPriority (pub usize);

/// Что надо по рабочим/оборудованию чтобы построить эту стационарку
pub fn stationary_requirements(
    target: Stationary,
) -> HashSet<TaskMeta> {
    match target {
        Stationary::BenchToolT1 => [
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(1000),
                stationary: Stationary::None,
            },
        ].iter().cloned().collect(),
        Stationary::BenchToolT2 => [
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ].iter().cloned().collect(),
        Stationary::BenchToolT3 => [
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ].iter().cloned().collect(),
        Stationary::FormatFurnace => [
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ].iter().cloned().collect(),
        Stationary::LabT1 => [
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ].iter().cloned().collect(),
        Stationary::LabT2 => [
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ].iter().cloned().collect(),
        Stationary::LabT3 => [
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ].iter().cloned().collect(),
        Stationary::Barrel => [
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ].iter().cloned().collect(),
        Stationary::NeuroTerminal => [
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ].iter().cloned().collect(),
        Stationary::None => HashSet::new (),
    }
}

/// Что надо по рабочим/оборудованию чтобы построить такую герму
pub fn germ_requirements(
    germ: Germ,
) -> HashSet<TaskMeta> {
    match germ {
        Germ::GermT1 => [
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ].iter().cloned().collect(),
        Germ::GermT2 => [
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ].iter().cloned().collect(),
        Germ::GermT3 => [
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ].iter().cloned().collect(),
    }
}

/// Чего не хватает
pub type WhatNotEnough = (
    HashSet<Stationary>,
    HashSet<(Profession, Tier)>,
    HashMap<Resource, RealUnits>,
    bool // Есть ли помещение для этого всего
);

/// Можем ли мы начать постройку этой стационарки
/// И чего нам не хватает для того чтобы построить
/// Ok(()) означает что всего хватает.
pub fn can_build_stationary (
    world: &mut World,
    exist_rsrcs: HashMap<Resource, RealUnits>,
    stationary: Stationary,
) -> Result<Entity, WhatNotEnough> {
    let requrements = stationary_requirements(stationary);
    let mut req_stnrs = HashSet::new();
    requrements
        .iter()
        .for_each(
            |req|{
                req_stnrs.insert(req.stationary);
            }
        );
    let mut req_ppl = HashSet::new();
    requrements
        .iter()
        .for_each(
            |req|
            {
                req_ppl.insert((req.prof, req.tier));
            }
        );
    let req_rsrcs = stationary_required_resources(stationary);

    let mut stat_query =
        <(&Stationary, &TaskStatus)>::query();
    let exist_stnrs: HashSet<Stationary> = stat_query
        .iter(world)
        .filter(
            |(_, status)|
            **status == TaskStatus::Ready
        ).map(|(stationary, _)| *stationary)
        .collect();
    let mut prof_query = <(&Profession, &Tier)>::query();
    let exist_ppl: HashSet<(Profession, Tier)> = prof_query
        .iter(world)
        .map(|(p, t)|(*p, *t))
        .collect();
    let room = get_sufficent_room(
        world,
        stationary_size(stationary),
        AreaType::Industrial,
    );
    let res_diff = what_not_enough(exist_rsrcs, req_rsrcs);
    if exist_stnrs.is_superset(&req_stnrs) &&
        exist_ppl.is_superset(&req_ppl) &&
        res_diff.is_empty() &&
        room.is_some()
    {
        Ok(match room {
            Some(a) => a,
            _ => unreachable!(),
        })
    } else {
        Err((
            diff2hset(req_stnrs.difference(&exist_stnrs)),
            diff2hset(req_ppl.difference(&exist_ppl)),
            res_diff,
            room.is_some()
        ))
    }
}

pub fn diff2hset<V>(
    diff: Difference<'_, V, RandomState>
) -> HashSet<V> where V: Copy + Eq + Hash {
    let mut result = HashSet::new();
    for v in diff {
        result.insert(*v);
    }
    result
}

/// Чего и сколько конкретно в правом хешмапе больше чем в левом
pub fn what_not_enough<K: Eq+Hash+Copy, V: Into<i32>+Ord+Sub<Output=V>+Copy> (
    exist: HashMap<K,V>,
    required: HashMap<K,V>,
) -> HashMap <K, V> {
    let mut result: HashMap <K, V> = HashMap::new();
    for (rk, rv) in required.iter() {
        match exist.get(rk) {
            None => {
                result.insert(*rk, *rv);
            },
            Some (lv) => {
                let minimum:i32 = 0;
                let right: i32 = (*rv).into();
                let left: i32 = (*lv).into();
                if (right - left) < minimum {
                    // меньше нуля - значит значение в левом хешмапе больше. Нам это не интересно.
                } else {
                    result.insert (*rk, *rv - *lv);
                }
            },
        }
    }
    result
}

/// Запустить постройку
/// предполагается что возможность постройки была проверена ранее
pub fn start_build_task(
    world: &mut World,
    stationary: Stationary,
    room: Entity,
    _priority: TaskPriority, // TODO: приоритет построек
) {
    let required_resources = stationary_required_resources(stationary);
    let _ = writeoff_bunch(world, required_resources);
    let requirements = stationary_requirements(stationary);
    world.push((
        stationary,
        TaskPriority(0),
        stationary_size(stationary),
        TaskStatus::Constructing,
        task_meta2progress(requirements),
        BelongsToRoom(room),
    ));
}

/// Что строится сейчас, и какой прогресс
pub fn currently_building (
    world: &mut World
) -> HashSet<(Stationary, TaskProgress)> {
    let mut result = HashSet::new();
    let mut query = <(&Stationary, &TaskProgress)>::query();
    for (stationary, progress) in query
        .iter(world)
    {
        result.insert((*stationary, progress.clone()));
    };
    result
}
