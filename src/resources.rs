use std::{
    fmt,
};
use std::hash::Hash;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, PartialOrd, Ord)]
pub enum Resource {
    Concentrat, // пищевой концентрат

    ScrapT1, // Лом черных металлов
    ScrapT2, // Лом цветных металлов
    ScrapT3, // Лом редких металлов

    ComponentT1, // механический компонент
    ComponentT2, // электронный компонент
    ComponentT3, // артефактный компонент

    ReagentT1, // экоцид - реактив разрушения
    ReagentT2, // компониум - реактив объединения
    ReagentT3, // сталий - реактив изменения.

    Concrete, // Бетонная крошка
    Slime, // Слизь
    Polymer, // Универсальный полимер
    BioRaw, // биологическое сырье.
}

/// FIXME: надо генерить список ресурсов напрямую из энума.
/// Хорошо если бы это появилось прям в расте, использовать sturm не хочется.
#[allow(dead_code)]
pub fn all_resources () -> Vec<Resource> {
    vec![
        Resource::Concentrat,
        Resource::ScrapT1,
        Resource::ScrapT2,
        Resource::ScrapT3,
        Resource::ComponentT1,
        Resource::ComponentT2,
        Resource::ComponentT3,
        Resource::ReagentT1,
        Resource::ReagentT2,
        Resource::ReagentT3,
        Resource::Concrete,
        Resource::Slime,
        Resource::Polymer,
        Resource::BioRaw,
    ]
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Resource::BioRaw           => write!(f, "{}", "Биосырье"),
            Resource::ScrapT1          => write!(f, "{}", "Чермет"),
            Resource::ScrapT2          => write!(f, "{}", "Цветмет"),
            Resource::ScrapT3          => write!(f, "{}", "Редкие металлы"),
            Resource::Slime => write!(f, "{}", "Слизь"),
            Resource::ComponentT1      => write!(f, "{}", "Механический компонент"),
            Resource::ComponentT2      => write!(f, "{}", "Электронный компонент"),
            Resource::ComponentT3      => write!(f, "{}", "Суперкомпонент"),
            Resource::Concrete      => write!(f, "{}", "Бетон"),
            Resource::ReagentT1        => write!(f, "{}", "Экоцид"),
            Resource::ReagentT2        => write!(f, "{}", "Компониум"),
            Resource::ReagentT3        => write!(f, "{}", "Сталий"),
            Resource::Polymer        => write!(f, "{}", "Полимер"),
            Resource::Concentrat     => write!(f, "{}", "Пищевой концентрат"),
        }
    }
}
