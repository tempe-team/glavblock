use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::min;

use legion::*;

use crate::core::*;
use crate::people::*;
use crate::production::*;
use crate::storage::*;
use crate::resources::*;

pub type BuildPowerPool = HashMap<Profession,HashMap<Tier, BuildPower>>;

pub fn turn(
    world: &mut World,
    resources: &mut Resources,
) {
    calc_buildpower(world, resources);
    process_tasks(world, resources);
    hunger_tick(world, resources);
    consume_concentrat(world, resources);
}

/// Сформировать пул билдпавера
fn calc_buildpower(
    world: &mut World,
    resources: &mut Resources,
) {
    let mut buildpower_pool = resources
        .get_mut::<BuildPowerPool>()
        .unwrap();
    let mut people_query = <(
        &Profession,
        &Tier,
    )>::query();
    for (prof, tier) in people_query.iter(world) {
        let human_bp = tier2comrad_buildpower(tier.clone());
        let by_tier_hm = buildpower_pool
            .entry(*prof)
            .or_insert(HashMap::new());

        let bp = by_tier_hm
            .entry(*tier)
            .or_insert(BuildPower(0));
        *bp += human_bp;
    }
}

/// Распределить все очки работы по заданиям
/// TODO: Наивная реализация. T3 инженеры на T3
/// станках тоже должны уметь делать T1 задания, причем
/// более эффективно чем T1 работяги на T1 станках.
/// Надо писать правила деградации.
pub fn process_tasks(
    world: &mut World,
    resources: &mut Resources,
) {
    let mut buildpower_pool = resources
        .get_mut::<BuildPowerPool>()
        .unwrap();

    // Смотрим готовые станки, которые мы можем использовать
    // для производства
    let mut stationary_query = <(
        &Stationary,
        &TaskStatus,
    )>::query();
    let mut stationaries:HashMap<Stationary, BuildPower> =
        HashMap::new();

    for (stat, status) in stationary_query.iter(world) {
        if (*status) == TaskStatus::Ready {
            let bp = stationary_build_power(*stat);
            let bp_for_update = stationaries
                .entry(*stat)
                .or_insert(BuildPower(0));
            *bp_for_update += bp;
        }
    };

    let mut query = <(
        Entity,
        &mut TaskStatus,
        &mut TaskProgress,
    )>::query();


    let mut delete_progresses = HashSet::new ();
    for (entity, status, progress) in query
        .iter_mut(world)
    {
        for (prof, tier, stationary, bp) in progress.who_should_finish.iter_mut() {
            // мощность станка которая у нас есть.
            let mut stat_bp_ = BuildPower(0);
            let stat_bp = stationaries.get_mut(stationary).unwrap_or(&mut stat_bp_);
            // мощность человечков, которая у нас есть
            let mut ppl_bp_ = BuildPower(0);
            let mut acc_ = HashMap::new();
            let ppl_bp = buildpower_pool
                .get_mut(prof)
                .unwrap_or(&mut acc_)
                .get_mut(tier)
                .unwrap_or(&mut ppl_bp_);
            // Какую по факту силу мы можем освоить
            let bp_to_withdraw = min(
                stat_bp.clone(),
                min(
                    ppl_bp.clone(),
                    bp.clone(),
                )
            );
            *stat_bp -= bp_to_withdraw;
            *ppl_bp -= bp_to_withdraw;
            *bp -= bp_to_withdraw;
            progress.bp_invested += bp_to_withdraw;
            if progress.bp_invested >= progress.bp_required {
                *status = TaskStatus::Ready;
                delete_progresses.insert(entity.clone());
            }
        }
    };
    for entity in delete_progresses.iter () {
        if let Some (mut entry) = world.entry(*entity) { entry.remove_component::<TaskProgress>() }
    }
}

/// Голод
pub fn hunger_tick(
    world: &mut World,
    _resources: &mut Resources,
) {
    let mut died_by_hunger: Vec<Entity> = Vec::new();
    let mut query = <(
        Entity,
        &mut Satiety,
        &mut Mood,
    )>::query();
    for (entity, sat, mood) in query.iter_mut(world) {
        sat.0 -= 10;
        if sat.0 < 10 {
            died_by_hunger.push(*entity);
        }
        // ниже ста - голод - минус настроение
        if sat.0 < 100 {
            mood.0.checked_sub(1);
        }
    }
    for e in died_by_hunger.iter() {
        world.remove(*e);
    }
}

/// Люди едят концентрат
pub fn consume_concentrat(
    world: &mut World,
    _resources: &mut Resources,
) {
    // сколько есть на складе
    let mut t1_conc_amount = how_much_we_have(
        world,
        Resource::Concentrat,
    );
    // Сколько выдано
    let mut t1_conc_writeroff = 0;
    // имеет настроение = человек.
    // да, знаю, зашибись признак.
    let mut query = <(
        &mut Mood,
        &mut Satiety,
    )>::query();

    for (mood, sat) in query.iter_mut(world){
        if t1_conc_amount.0 <= 0 {
            // Не дали пожрать. Настроение
            // от такого ухудшается.
            mood.0.checked_sub(1);
        } else {
            t1_conc_amount.0 -= 1;
            t1_conc_writeroff += 1;
            mood.0 += 1;
            sat.0 += 10;
        }
    }
    let rest = writeoff(
        world,
        Resource::Concentrat,
        RealUnits(t1_conc_writeroff),
    );
    // не получилось списать все. Ну ок, списываем что есть.
    if rest.0 != 0 {
        writeoff(
            world,
            Resource::Concentrat,
            rest,
        );
    }
}
