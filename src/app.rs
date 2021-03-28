use eframe::{egui, epi};

use legion::*;

use crate::screens::*;
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

    let cell_sciencists = install_germ(
        world,
        Tier::T1,
        AreaType::Living,
    );
    spawn_comrad(
        world,
        Profession::Scientist,
        Tier::T1,
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
                cell,
            );
        }
    };

    // Ресурсы
    assert_eq!(
        RealUnits(0),
        put_resource(
            world,
            Resource::Concentrat,
            RealUnits(1000),
        ),
    );
    assert_eq!(
        RealUnits (0),
        put_resource(
            world,
            Resource::ScrapT1,
            RealUnits(50),
        )
    );
    assert_eq!(
        RealUnits (0),
        put_resource(
            world,
            Resource::ScrapT2,
            RealUnits(40),
        )
    );
    assert_eq!(
        RealUnits (0),
        put_resource(
            world,
            Resource::Polymer,
            RealUnits(30),
        )
    );
}

impl Default for GlavblockApp {
    fn default() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();
        resources.insert(BuildPowerPool::new()
        );
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
    /// Put your widgets into a Symbol’s value as variable is void: SidePanel, Symbol’s value as variable is void: TopPanel, Symbol’s value as variable is void: CentralPanel, Symbol’s value as variable is void: Window or Symbol’s value as variable is void: Area.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>)  {
        draw_ui(
            &mut self.world,
            &mut self.resources,
            ctx,
            frame,
        )
    }

    fn setup(&mut self, _ctx: &egui::CtxRef) {}

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn on_exit(&mut self) {}

    fn initial_window_size(&self) -> Option<egui::Vec2> {
        None
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn is_resizable(&self) -> bool {
        true
    }

    fn max_size_points(&self) -> egui::Vec2 {
        // Some browsers get slow with huge WebGL canvases, so we limit the size:
        egui::Vec2::new(1024.0, 2048.0)
    }

    fn clear_color(&self) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        egui::Color32::from_rgb(12, 12, 12).into()
    }

    fn icon_data(&self) -> Option<epi::IconData> {
        None
    }
}
