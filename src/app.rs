use std::sync::mpsc;

use eframe::{
    egui::{
        TextureId,
        CtxRef,
        SidePanel,
        Button,
        ImageButton,
        CentralPanel,
        CollapsingHeader,
        Vec2,
        Rgba,
        Color32,
    },
    epi,
};

use std::collections::HashMap;
use std::vec::Vec;

use legion::*;

use crate::core::*;
use crate::production::*;
use crate::resources::*;
use crate::storage::*;
use crate::people::*;
use crate::turn::*;
use crate::area::*;
use crate::assets::{
    fetch_resource_with_check,
    decode_textures,
    get_texture_id,
};

fn init_colony(world: &mut World) {
    // казарма с рассчетом №1-Ж
    let barracks = install_germ(
        world,
        Germ::GermT2,
        AreaType::Military,
    );
    spawn_1_g(world, barracks);

    // T2 производственное помещение под установку верстака, станка, печи, и чанов
    let _manufactory = install_germ(
        world,
        Germ::GermT2,
        AreaType::Industrial,
    );

    // T2 Склад с чанами и стеллажами
    let _stock = install_germ(
        world,
        Germ::GermT2,
        AreaType::Party,
    );

    // Т1 комнатка для исследований
    install_germ(
        world,
        Germ::GermT1,
        AreaType::Science,
    );

    let cell_sciencists = install_germ(
        world,
        Germ::GermT1,
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
            Germ::GermT1,
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

pub enum ScreenId {
    ScreenResources,
    ScreenDemography,
    ScreenSpace,
    ScreenTasks,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct GlavblockApp {
    pub label: String,
    pub world: World,
    pub resources: Resources,
    pub textures: HashMap<String, TextureId>,
    pub resource_loaders: HashMap<String, mpsc::Receiver<Vec<u8>>>,
    pub current_screen: ScreenId,
}

impl GlavblockApp {
    fn draw_ui(
        &mut self,
        ctx: &CtxRef,
        frame: &mut epi::Frame<'_>,
    ) {
        fetch_resource_with_check(
            &mut self.resource_loaders,
            &mut self.textures,
            "assets/resources.png".to_string(),
            frame,
        );
        fetch_resource_with_check(
            &mut self.resource_loaders,
            &mut self.textures,
            "assets/demography.png".to_string(),
            frame,
        );
        fetch_resource_with_check(
            &mut self.resource_loaders,
            &mut self.textures,
            "assets/space.png".to_string(),
            frame,
        );
        fetch_resource_with_check(
            &mut self.resource_loaders,
            &mut self.textures,
            "assets/tasks.png".to_string(),
            frame,
        );
        decode_textures(
            &mut self.resource_loaders,
            &mut self.textures,
            frame,
        );

        SidePanel::left("left_panel", 80.0).show(ctx, |ui| {
            let button_txtr_size = [64.0, 64.0];

            if ui
                .add(ImageButton::new(
                    get_texture_id(
                        &mut self.textures,
                        "assets/resources.png".to_string(),
                    ),
                    button_txtr_size,
                )).on_hover_text("Ресурсы")
                .clicked()
            {
                self.current_screen = ScreenId::ScreenResources;
            }
            if ui
                .add(ImageButton::new(
                    get_texture_id(
                        &mut self.textures,
                        "assets/demography.png".to_string(),
                    ),
                    button_txtr_size,
                )).on_hover_text("Демография")
                .clicked()
            {
                self.current_screen = ScreenId::ScreenDemography;
            }
            if ui
                .add(ImageButton::new(
                    get_texture_id(
                        &mut self.textures,
                        "assets/space.png".to_string(),
                    ),
                    button_txtr_size,
                )).on_hover_text("Пространство")
                .clicked()
            {
                self.current_screen = ScreenId::ScreenSpace;
            }
            if ui
                .add(ImageButton::new(
                    get_texture_id(
                        &mut self.textures,
                        "assets/tasks.png".to_string(),
                    ),
                    button_txtr_size,
                ))
                .on_hover_text("Производство")
                .clicked()
            {
                self.current_screen = ScreenId::ScreenTasks;
            }
        });
        let mut container = CentralPanel::default();
        match self.current_screen {
            ScreenId::ScreenResources => {
                self.resources_screen(ctx)
            },
            ScreenId::ScreenDemography => {}
            ScreenId::ScreenSpace => {}
            ScreenId::ScreenTasks => {
                self.tasks_screen(ctx)
            },
        }
    }

    fn resources_screen(
        &mut self,
        ctx: &CtxRef,
    ) {
        let mut container = CentralPanel::default();
        let resources = what_we_have(&mut self.world);
        container.show(ctx, |ui| {
            CollapsingHeader::new("Ресурсы")
                .default_open (true)
                .show(
                    ui,
                    |ui| {
                        for res in all_resources().iter () {
                        let cnt = resources
                                .get(&res)
                                .unwrap_or(&RealUnits(0));
                            ui.label(
                                &format!("{}: {}", res, cnt.0),
                            );
                        }
                    }
                );
            if ui.button("Смена").clicked() {
                turn(&mut self.world, &mut self.resources);
            }
        });
    }

    fn tasks_screen (
        &mut self,
        ctx: &CtxRef,
    ) {
        CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, | cols| {
                // отрисовать CollapsedHeader-ами все доступные стационарки, гермы, ресурсы, изделия
                // недоступные к постройке отрисовывать красным
                // в ховере отрисовывать требуемые ресурсы
                CollapsingHeader::new("Строить!")
                    .default_open (true)
                    .show(
                        &mut cols[0],
                        |ui| {
                            let exists_rsrs = what_we_have(&mut self.world);
                            for stat in all_stationaries().iter() {
                                let stat_meta = can_build_stationary (
                                    &mut self.world,
                                    exists_rsrs.clone(), //FIXME
                                    *stat,
                                );
                                match stat_meta {
                                    Ok(room) =>  if ui.add(
                                        Button::new(&format!("{}", *stat))
                                    ).on_hover_ui(
                                        |ui| {
                                            for req in stationary_requirements(*stat).iter().map(display_task_meta) {
                                                ui.label(req);
                                            }
                                        }
                                    ).clicked () {
                                        start_build_task(
                                            &mut self.world,
                                            *stat,
                                            room,
                                            TaskPriority (0),
                                        );
                                    },
                                    Err((
                                        not_enough_stts,
                                        not_enough_ppl,
                                        not_enough_rsrcs,
                                        is_enough_space
                                    )) => {
                                        ui.add(
                                            Button::new(&format!("{}", *stat)).text_color(Color32::RED)
                                        ).on_hover_ui(
                                            |ui| {
                                                ui.label("Не хватает:");
                                                for v in not_enough_stts.iter() {
                                                    ui.label(format!("{}", *v));
                                                }
                                                for v in not_enough_ppl.iter() {
                                                    let v_ = *v;
                                                    ui.label(format!("{}, {}", v_.0, v_.1));
                                                }
                                                for v in not_enough_rsrcs.iter() {
                                                    ui.label(format!("{}, {}", v.0, v.1.0));
                                                }
                                                if !is_enough_space {
                                                    ui.label("А еще места нет");
                                                }
                                            }
                                        );
                                    }
                                }

                            }
                        }
                    );
                let in_progress = currently_building(&mut self.world);
                let mut right = cols.get_mut(1).unwrap();

                for ip in in_progress.iter () {
                    right.label(format!("{} , {}", ip.0, ip.1));
                }
            });

            if ui.button("Смена").clicked() {
                turn(&mut self.world, &mut self.resources);
            };
        });
    }
}

impl Default for GlavblockApp {
    fn default() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();

        // Заглушка для техпроцессов, чтобы алгоритмы подсчета не орали что у тебя нету постройки "Stationary::None"
        // которая на самом деле означает отсутствие станка
        world.push((
            Stationary::None,
            stationary_size(Stationary::None),
            TaskStatus::Ready,
        ));
        let resource_loaders = HashMap::new ();
        let textures = HashMap::new ();
        let current_screen = ScreenId::ScreenResources;
        resources.insert(BuildPowerPool::new());
        init_colony(&mut world);
        Self {
            // Example stuff:
            label: "Главблок!".to_owned(),
            world,
            resources,
            textures,
            resource_loaders,
            current_screen,
        }
    }
}

impl epi::App for GlavblockApp {
    fn name(&self) -> &str {
        &self.label
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

    fn update(&mut self, ctx: &CtxRef, frame: &mut epi::Frame<'_>)  {
        self.draw_ui(ctx, frame)
    }

    fn setup(&mut self, _ctx: &CtxRef) {
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn on_exit(&mut self) {}

    fn initial_window_size(&self) -> Option<Vec2> {
        None
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn is_resizable(&self) -> bool {
        true
    }

    fn max_size_points(&self) -> Vec2 {
        // Some browsers get slow with huge WebGL canvases, so we limit the size:
        Vec2::new(1024.0, 2048.0)
    }

    fn clear_color(&self) -> Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        Color32::from_rgb(12, 12, 12).into()
    }

    fn icon_data(&self) -> Option<epi::IconData> {
        None
    }

}
