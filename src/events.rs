use legion::*;

use std::collections::{
    HashMap,
    HashSet,
};

use crate::core::*;
use crate::resources::*;
use crate::people::*;
use crate::storage::*;

/// Идентификатор эпизода - для перелинковки
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EpisodeId(String);

/// тут описано кто участвовал в создании конкретного экрана
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Authors {
    pub painters: Vec<String>,  // кто сделал арт
    pub writers: Vec<String>,   // кто написал сюжет
    pub scripters: Vec<String>, // кто завернул все в код и теперь поддерживает

    // Как делить бабло. Цифры определяют какую долю получат
    // художник(и), писател(и) и программер(ы).
    // Допустим на эпизод упало 100 рублей.
    // Тогда берется общая сумма to_all = to_painters + to_writers + to_scripters
    // и количество бабла которое уходит например художникам =
    // 100 рублей * (to_painters / to_all)
    pub to_painters: usize,
    pub to_writers: usize,
    pub to_scripters: usize,
}

/// Условие наступления события
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Condition {
    NoCondition, // может случится без ограничений
    AfterNTurn(usize), // Не раньше n хода
    BeforeNTurn(usize), // Не позже n хода
    AfterNSamosbor(usize), // Не раньше чем случиться n по порядку самосбор
    EpisodeHappened(EpisodeId), // Зависимость от конкретного эпизода
    TagHappened(EpisodeTag), // Зависимость от конкретного тега
}

/// Метки для классификации событий
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EpisodeTag {
    NoType, // Тип события не имеет значения
    Samosbor, // Самосбор
    Migration, // Пришли мигранты
    Infestation, // Заражение
}

/// Эффект от того что юзер пришел на событие
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SideEffect {
    GameOver((
        FilePath, // картинка геймовера
        String, // текст геймовера
    )),
    AddResource((Resource, RealUnits)), // Добавить ресурсы
    DropResource((Resource, RealUnits)), // оформить потерю ресурсов юзеру
    SpawnComrads(( // доселить человечков по жилячейкам
        Profession,
        Tier,
        usize, // сколько человечков доселить
    )),
}

/// Один конкретный экран
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Episode {
    id: EpisodeId,
    is_reusable: bool, // может случиться многократно или однократно
    tags: Vec<EpisodeTag>,
    probability: u8, // Вероятность возникновения события. 100 и более это 100%
    authors: Authors,
    picture: String, // идентификатор картинки. Путь к ассету.
    conditions: Vec<Condition>, // Условия по которым событие допускается
    description: String, // Текстовое содержание
    branches: HashMap<String, Option<EpisodeId>>, // Варианты развития. None значит выход из квеста
    side_effects: Vec<SideEffect>, // Что происходит с миром когда игрок переходит на это событие
}

/// Выполняется ли это условие на данном ходу
pub fn is_condition_satisfy(
    _world: &mut World,
    resources: &mut Resources,
    condition: Condition,
) -> bool {
    match condition {
        Condition::NoCondition => true,
        Condition::AfterNTurn(turn) => {
            let current_turn = resources.get::<TurnCounter>().unwrap();
            current_turn.0 > turn
        },
        Condition::BeforeNTurn(turn) => {
            let current_turn = resources.get::<TurnCounter>().unwrap();
            current_turn.0 < turn
        },
        Condition::AfterNSamosbor(smsbr) => {
            let smsbrz = resources.get::<SamosborCounter>().unwrap();
            smsbrz.0 > smsbr
        },
        Condition::EpisodeHappened(episode_id) => {
            let episodes = resources.get::<HashSet<EpisodeId>>().unwrap();
            episodes.contains(&episode_id)
        },
        Condition::TagHappened(tag) => {
            let tags = resources.get::<HashSet<EpisodeTag>>().unwrap();
            tags.contains(&tag)
        },
    }
}

pub fn load_episodes (
    _world: &mut World,
    _resources: &mut Resources,
    episodes: Vec<Episode>,
) {
    for _episode in episodes {
    }
}

pub fn eval_side_effects (
    _world: &mut World,
    _resources: &mut Resources,
    _sf: SideEffect,
) {
}
