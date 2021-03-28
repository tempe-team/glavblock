use eframe::{egui, epi};
use legion::*;

use crate::core::*;
use crate::production::*;
use crate::resources::*;
use crate::storage::*;
use crate::people::*;
use crate::turn::*;
use crate::area::*;

pub fn draw_ui(
    world: &mut World,
    ecs_resources: &mut Resources,
    ctx: &egui::CtxRef,
    _frame: &mut epi::Frame<'_>,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("Смена").clicked() {
            turn(world, ecs_resources);
        }
        ui.separator();
        ui.heading("Люди");
        let people: std::collections::HashMap<Profession, usize> = people_by_profession(world);
        if let Some(cnt) = people.get(&Profession::Likvidator) {
            ui.label(
                &format!(
                    "{}: {}",
                    Profession::Likvidator,
                    cnt,
                ),
            );
        };
        if let Some(cnt) = people.get(&Profession::Scientist) {
            ui.label(
                &format!(
                    "{}: {}",
                    Profession::Scientist,
                    cnt,
                ),
            );
        };
        if let Some(cnt) = people.get(&Profession::Worker) {
            ui.label(
                &format!("{}: {}", Profession::Worker, cnt),
            );
        };
        ui.separator();
        ui.heading("Ресурсы");
        let resources_ = what_we_have(world);
        let mut resources: Vec<(&Resource, &RealUnits)> = resources_.iter().collect();
        resources.sort_by(|(r1, _), (r2, _)| r1.cmp (r2));
        for (res, cnt) in resources.iter() {
            ui.label(
                &format!("{}: {}", res, cnt.0),
            );
        }
        ui.separator();
        ui.heading("Настроение");
                                                                 let mood: usize = block_mood(world);
        ui.label(
            &format!("Среднее настроение: {}", mood.checked_div(people.len()).unwrap_or(1)),
        );
        let satiety: Satiety = block_satiety(world);
        ui.label(
            &format!(
                "Сытость: {}",
                satiety.0.checked_div(people.len() as u16).unwrap_or(1)),
        );
        ui.heading("Пространство");
        let mut rooms =
            rooms_with_space(world);
        rooms.sort_by(
            |(_, r1), (_, r2)|
            r2.0.cmp(&(r1.0))
        );
        for (room, space) in rooms.iter() {
            ui.label(
                &format!("{}: {}", room, space.0),
            );
        }
    });
}
