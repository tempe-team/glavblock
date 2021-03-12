use eframe::{egui, epi};

use legion::*;

use crate::core::*;
use crate::production::*;
use crate::resources::*;
use crate::storage::*;
use crate::people::*;
use crate::turn::*;
use crate::area::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct GlavblockApp {
    // Example stuff:
    pub label: String,
    pub world: World,
    pub resources: Resources,
}

fn init_colony(world: &mut World) {
    // казарма с рассчетом №1-Ж
    let barracks = install_germ(
        world,
        Tier::T2,
        AreaType::Military,
    );
    spawn_1_g(world, barracks);

    // T2 производственное помещение под установку верстака, станка, печи, и чанов
    let _manufactory = install_germ(
        world,
        Tier::T2,
        AreaType::Industrial,
    );

    // T2 Склад с чанами и стеллажами
    let _stock = install_germ(
        world,
        Tier::T2,
        AreaType::Party,
    );

    // Т1 комнатка для исследований
    install_germ(
        world,
        Tier::T1,
        AreaType::Science,
    );

    let start_sci_spec = random_sci_spec();
    let cell_sciencists = install_germ(
        world,
        Tier::T1,
        AreaType::Living,
    );
    spawn_comrad(
        world,
        Profession::Scientist,
        Tier::T1,
        MilitaryDep::None,
        start_sci_spec,
        cell_sciencists,
    );

    // Жилячейки
    for _ in 0..33 {
        let cell = install_germ(
            world,
            Tier::T1,
            AreaType::Living,
        );
        for _ in 0..3 {
            spawn_comrad(
                world,
                Profession::Worker,
                Tier::T1,
                MilitaryDep::None,
                SciSpec::None,
                cell,
            );
        }
    };

    // Ресурсы

    put_resource(
        world,
        Resource::ConcentratT1,
        RealUnits(100),
    );

    put_resource(
        world,
        Resource::ConcentratT1,
        RealUnits(1000),
    );
    put_resource(
        world,
        Resource::ScrapT1,
        RealUnits(500),
    );
    put_resource(
        world,
        Resource::ScrapT2,
        RealUnits(50),
    );

    put_resource(
        world,
        Resource::PolymerT1,
        RealUnits(100),
    );
    put_resource(
        world,
        Resource::PolymerT2,
        RealUnits(10),
    );
}

impl Default for GlavblockApp {
    fn default() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();
        resources.insert(BuildPowerPool::new());
        init_colony(&mut world);
        Self {
            // Example stuff:
            label: "Главблок".to_owned(),
            world,
            resources,
        }
    }
}

impl epi::App for GlavblockApp {
    fn name(&self) -> &str {
        "egui template"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.separator();
            ui.heading("Люди");
            let people: std::collections::HashMap<Profession, usize> = people_by_profession(&mut self.world);

            people.iter().for_each(|(prof, count)| {
                ui.label(
                    &format!("{}: {}", *prof, *count),
                );
            });

            ui.separator();
            ui.heading("Ресурсы");
            for (res, count) in what_we_have(&mut self.world).iter() {
                ui.label(
                    &format!("{}: {}", *res, (*count).0),
                );
            };
            ui.separator();
            let mood: usize = block_mood(&mut self.world);
                ui.label(
                    &format!("Среднее настроение: {}", mood / people.len()),
                );
            let satiety: Satiety = block_satiety(&mut self.world);
                ui.label(
                    &format!("Сытость: {}", satiety.0 as usize / people.len()),
                );

            if ui.button("Смена").clicked() {
                turn(&mut self.world, &mut self.resources);
            }
        });

    }
}
