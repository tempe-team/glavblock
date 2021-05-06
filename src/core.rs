use std::hash::Hash;
use std::fmt;
use rand::thread_rng;
use rand::Rng;

/// Бросить кубы
pub fn _d(rolls:u8, sides:u8) -> usize {
    if sides < 1 || rolls < 1 {
        0
    } else {
        let mut rng = thread_rng();
        let mut result = 0;
        for _ in 0..rolls {
            result += rng.gen_range(0..sides) as usize
        }
        result
    }
}

pub enum SamosborError {
    NoEmptyArea,
    NotEnoughArea,
    NotEnoughResources,
}

/// Уровень(изделия, опыта, ресурса и тп)
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, PartialOrd, Ord)]
pub enum Tier {
    NoTier, // уникальные штуки
    T1,
    T2,
    T3,
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tier::NoTier => write!(f, "{}", "без разряда"),
            Tier::T1     => write!(f, "{}", "1 разряда"),
            Tier::T2     => write!(f, "{}", "2 разряда"),
            Tier::T3     => write!(f, "{}", "3 разряда"),
        }
    }
}

/// В каком состоянии строение
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TaskStatus {
    Constructing, // Строится
    Ready, // Готово
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Constructing => write!(f, "{}", "Строится"),
            TaskStatus::Ready        => write!(f, "{}", "Готово"),
        }
    }
}
