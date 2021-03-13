use std::fmt;
use std::hash::Hash;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, PartialOrd, Ord)]
pub enum Resource {
    BioRawT1, // загрязненное биологическое сырье.
    BioRawT2, // чистое биологическое сырье.
    BioRawT3, // очищенное биологическое сырье.
    ScrapT1, // Лом черных металлов
    ScrapT2, // Лом цветных металлов
    ScrapT3, // Лом редких металлов
    Concrete, // Бетонная крошка
    IsoConcrete, // Изобетон. Артефактный ресурс.

    TransparentSlime, // Прозрачная слизь
    BlackSlime, // Черная слизь
    BrownSlime, // Коричневая слизь
    RedSlime,   // Красная слизь
    PinkSlime,  // Розовая слизь
    WhiteSlime, // Белая слизь. Артефактный ресурс.

    ComponentT1, // механический компонент
    ComponentT2, // электронный компонент
    ComponentT3, // артефактный компонент

    ReagentT1, // экоцид - реактив разрушения
    ReagentT2, // компониум - реактив объединения
    ReagentT3, // сталий - реактив изменения.

    PolymerT1, // Синтетическая ткань
    PolymerT2, // пластик
    PolymerT3, // супер пластик

    ConcentratT1, // белый пищевой концентрат
    ConcentratT2, // черный пищевой концентрат
    ConcentratT3, // красный пищевой концентрат
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Resource::BioRawT1         => write!(f, "{}", "Загрязненное биосырье"),
            Resource::BioRawT2         => write!(f, "{}", "Биосырье"),
            Resource::BioRawT3         => write!(f, "{}", "Очищенное биосырье"),
            Resource::ScrapT1          => write!(f, "{}", "Чермет"),
            Resource::ScrapT2          => write!(f, "{}", "Цветмет"),
            Resource::ScrapT3          => write!(f, "{}", "Редкие металлы"),
            Resource::TransparentSlime => write!(f, "{}", "Прозрачная слизь"),
            Resource::BlackSlime       => write!(f, "{}", "Черная слизь"),
            Resource::BrownSlime       => write!(f, "{}", "Коричневая слизь"),
            Resource::RedSlime         => write!(f, "{}", "Красная слизь"),
            Resource::PinkSlime        => write!(f, "{}", "Розовая слизь"),
            Resource::WhiteSlime       => write!(f, "{}", "Белая слизь"),
            Resource::ComponentT1      => write!(f, "{}", "Механический компонент"),
            Resource::ComponentT2      => write!(f, "{}", "Электронный компонент"),
            Resource::ComponentT3      => write!(f, "{}", "Суперкомпонент"),
            Resource::Concrete      => write!(f, "{}", "Бетон"),
            Resource::IsoConcrete      => write!(f, "{}", "ИзоБетон"),
            Resource::ReagentT1        => write!(f, "{}", "Экоцид"),
            Resource::ReagentT2        => write!(f, "{}", "Компониум"),
            Resource::ReagentT3        => write!(f, "{}", "Сталий"),
            Resource::PolymerT1        => write!(f, "{}", "Синтетическая ткань"),
            Resource::PolymerT2        => write!(f, "{}", "Пластик"),
            Resource::PolymerT3        => write!(f, "{}", "Суперпластик"),
            Resource::ConcentratT1     => write!(f, "{}", "Белый концентрат"),
            Resource::ConcentratT2     => write!(f, "{}", "Черный концентрат"),
            Resource::ConcentratT3     => write!(f, "{}", "Красный концентрат"),
        }
    }
}
