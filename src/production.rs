use legion::*;
use std::ops::*;
use std::hash::Hash;
use crate::core::*;
use crate::area::*;
use crate::people::*;
use crate::resources::*;
use crate::storage::*;

use std::collections::HashMap;

/// Приоритет задачи
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority(pub usize);

/// Метка того, к какому стационарному объекту принадлежит эта штука
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BelongsToStationary (pub Entity);

/// Стационарные объекты
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

/// Гермкомплект. Инфраструктура конкертного помещения. Бывает T1, T2, T3.
pub struct Germ ();

/// В каком состоянии строение
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StationaryStatus {
    Constructing, // Строится
    Ready, // Готово
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
        Stationary::Rack => AreaOccupied(500),
        Stationary::NeuroTerminal => AreaOccupied(500),
    }
}

/// Поставить герму + обустроить помещение
pub fn install_germ(
    world: &mut World,
    tier: Tier,
    purpose: AreaType,
) -> Entity {
    let t: usize = match tier {
        Tier::T1     => 1,
        Tier::T2     => 2,
        Tier::T3     => 3,
        Tier::NoTier => 0,
    };
    world.push((
        Germ(),
        tier,
        StationaryStatus::Constructing,
        germ_requirements(tier),
        purpose,
        tier2germ_capacity(tier),
    ))
}

/// Вместимость гермы
fn tier2germ_capacity(tier: Tier) -> AreaCapacity {
    match tier {
        Tier::NoTier => unimplemented!(),
        Tier::T1 => AreaCapacity(3000),
        Tier::T2 => AreaCapacity(15000),
        Tier::T3 => AreaCapacity(50000),
    }
}

/// Количество труда, которое должен затратить (затратил)
/// работник на выполнение задачи за одну смену
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct BuildPower(pub usize);

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
pub fn tier2comrad_buildpower (tier: Tier) -> BuildPower {
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
        Stationary::None => BuildPower(0),
        Stationary::BenchToolT1 => BuildPower(10),
        Stationary::BenchToolT2 => BuildPower(20),
        Stationary::BenchToolT3 => BuildPower(40),
        Stationary::FormatFurnace => BuildPower(10),
        Stationary::LabT1 => BuildPower(10),
        Stationary::LabT2 => BuildPower(10),
        Stationary::LabT3 => BuildPower(10),
        Stationary::Barrel => BuildPower(10),
        Stationary::Rack => BuildPower(0),
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
        Stationary::Rack => [
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
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct TaskMeta {
    pub prof: Profession,
    pub tier: Tier, // Тир исполнителя
    pub bp: BuildPower,
    pub stationary: Stationary, // на каком оборудовании надо выполнять работу
}

/// Приоритет задачи
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct TaskPriority (pub usize);

/// Что надо по рабочим/оборудованию чтобы построить эту стационарку
pub fn stationary_requirements(
    target: Stationary,
) -> Vec<TaskMeta> {
    match target {
        Stationary::BenchToolT1 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ],
        Stationary::BenchToolT2 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ],
        Stationary::BenchToolT3 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ],
        Stationary::FormatFurnace => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ],
        Stationary::LabT1 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ],
        Stationary::LabT2 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ],
        Stationary::LabT3 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ],
        Stationary::Barrel => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ],
        Stationary::Rack => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ],
        Stationary::NeuroTerminal => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ],
        Stationary::None => Vec::new (),
    }
}

/// Что надо по рабочим/оборудованию чтобы построить такую герму
pub fn germ_requirements(
    tier: Tier,
) -> Vec<TaskMeta> {
    match tier {
        Tier::NoTier => Vec::new(),
        Tier::T1 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ],
        Tier::T2 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ],
        Tier::T3 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
            },
        ],
    }
}

/// Запустить постройку
pub fn start_build_task (
    world: &mut World,
    stationary: Stationary,
    room: Entity,
    priority: TaskPriority,
) -> Result<(), SamosborError> {
    let free_space = get_room_free_space(world, room);
    let required_space = stationary_size(stationary);
    if free_space.0 < required_space.0 {
        Err(SamosborError::NotEnoughArea)
    } else {
        let required_resources = stationary_required_resources(stationary);
        let _ = writeoff_bunch(world, required_resources)?;
        let task_id = world.push((
            stationary,
            stationary_size(stationary),
            StationaryStatus::Constructing,
            BelongsToRoom(room),
        ));
        let requirements = stationary_requirements(stationary);
        for task_meta in requirements.iter() {
            world.push((
                BelongsToStationary(task_id),
                task_meta.clone(),
                priority,
            ));
        };
        Ok (())
    }
}
