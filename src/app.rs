use std::{
    sync::mpsc,
    collections::{
        HashMap,
        HashSet,
    },
    vec::Vec,
};

use eframe::{
    egui::{
        Align,
        TextureId,
        CtxRef,
        Direction,
        SidePanel,
        Button,
        ImageButton,
        CentralPanel,
        CollapsingHeader,
        Vec2,
        Rgba,
        Color32,
        Layout,
        vec2,
    },
    epi,
};

use legion::*;

use crate::core::*;
use crate::events::*;
use crate::production::*;
use crate::resources::*;
use crate::storage::*;
use crate::people::*;
use crate::turn::*;
use crate::area::*;
use crate::assets::{
    load_all_resources,
    decode_textures,
    get_texture_id,
};
use crate::queries::{
    who_take_place,
    what_take_place,
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

/// На каком экране мы сейчас
pub enum ScreenId {
    ScreenResources,
    ScreenDemography,
    ScreenSpace,
    ScreenTasks,
}

/// Стейт интерфейса пространства.
pub struct SpaceScreenState {
    pub living_checkbox: bool,
    pub military_checkbox: bool,
    pub party_checkbox: bool,
    pub science_checkbox: bool,
    pub industrial_checkbox: bool,
    pub selected_area: Option<Entity>,
}

impl Default for SpaceScreenState {
    fn default() -> Self {
        Self {
            living_checkbox: true,
            military_checkbox: true,
            party_checkbox: true,
            science_checkbox: true,
            industrial_checkbox: true,
            selected_area: None,
        }
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct GlavblockApp {
    pub label: String,
    pub world: World,
    pub resources: Resources,
    pub textures: HashMap<String, TextureId>,
    pub resource_loaders: HashMap<String, mpsc::Receiver<Vec<u8>>>,
    pub current_screen: ScreenId,
    pub space_screen: SpaceScreenState,
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
        let space_screen = SpaceScreenState::default();
        resources.insert(BuildPowerPool::new());
        resources.insert(SamosborCounter(0));
        resources.insert(TurnCounter(0));
        let episodes_happened: HashSet<EpisodeId> = HashSet::new();
        resources.insert(episodes_happened);
        let tags_happened: HashSet<EpisodeTag> = HashSet::new();
        resources.insert(tags_happened);
        init_colony(&mut world);
        Self {
            // Example stuff:
            label: "Главблок!".to_owned(),
            world,
            resources,
            textures,
            resource_loaders,
            current_screen,
            space_screen,
        }
    }
}

impl GlavblockApp {
    fn draw_ui(
        &mut self,
        ctx: &CtxRef,
        frame: &mut epi::Frame<'_>,
    ) {
        load_all_resources(
            &mut self.resource_loaders,
            &mut self.textures,
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
        match self.current_screen {
            ScreenId::ScreenResources =>
                self.resources_screen(ctx),
            ScreenId::ScreenDemography =>
                self.demography_screen(ctx),
            ScreenId::ScreenSpace =>
                self.space_screen(ctx),
            ScreenId::ScreenTasks =>
                self.tasks_screen(ctx),
        }
    }

    fn resources_screen(
        &mut self,
        ctx: &CtxRef,
    ) {
        let resources = what_we_have(&mut self.world);
        CentralPanel::default().show(ctx, |ui| {
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
            ui.separator ();
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
                let mut in_progress:Vec<(Stationary, TaskProgress)>  = currently_building(&mut self.world)
                    .iter()
                    .cloned()
                    .collect();
                in_progress.sort();
                let right = cols.get_mut(1).unwrap();

                for ip in in_progress.iter () {
                    right.label(format!("{} , {}", ip.0, ip.1));
                }
            });

            ui.separator();
            if ui.button("Смена").clicked() {
                turn(&mut self.world, &mut self.resources);
            };
        });
    }

    fn space_screen (
        &mut self,
        ctx: &CtxRef,
    ) {
        CentralPanel::default().show(ctx, |ui| {
            ui.allocate_ui_with_layout(
                vec2(
                    ui.available_size_before_wrap_finite().x,
                    32.0
                ),
                Layout::from_main_dir_and_cross_align(
                    Direction::LeftToRight,
                    Align::Center,
                ),
                |ui| {
                    let img_size = [32.0, 32.0];
                    ui.image(
                        get_texture_id(
                            &mut self.textures,
                            "assets/living.png".to_string(),
                        ),
                        img_size,
                    );
                    ui.checkbox(
                        &mut self.space_screen.living_checkbox,
                        "",
                    );
                    ui.image(
                        get_texture_id(
                            &mut self.textures,
                            "assets/military.png".to_string(),
                        ),
                        img_size,
                    );
                    ui.checkbox(
                        &mut self.space_screen.military_checkbox,
                        "",
                    );
                    ui.image(
                        get_texture_id(
                            &mut self.textures,
                            "assets/party.png".to_string(),
                        ),
                        img_size,
                    );
                    ui.checkbox(
                        &mut self.space_screen.party_checkbox,
                        "",
                    );
                    ui.image(
                        get_texture_id(
                            &mut self.textures,
                            "assets/science.png".to_string(),
                        ),
                        img_size,
                    );
                    ui.checkbox(
                        &mut self.space_screen.science_checkbox,
                        "",
                    );
                    ui.image(
                        get_texture_id(
                            &mut self.textures,
                            "assets/industrial.png".to_string(),
                        ),
                        img_size,
                    );
                    ui.checkbox(
                        &mut self.space_screen.industrial_checkbox,
                        "",
                    );
                }
            );

            let rooms = all_rooms_with_space(&mut self.world);
            let people = who_take_place(
                &mut self.world,
            );
            let stationaries = what_take_place(
                &mut self.world,
            );
            ui.columns(
                2,
                |cols| {
                    cols[0].allocate_ui_with_layout(
                        vec2(
                            cols[0].available_size_before_wrap_finite().x,
                            cols[0].available_size_before_wrap_finite().y,
                        ),
                        Layout::from_main_dir_and_cross_align(
                            Direction::TopDown,
                            Align::LEFT,
                        ),
                        |ui| {
                            // Площади каких назначений мы отображаем (в зависимости от тыкнутых галок)
                            let mut include_purposes = HashSet::new();
                            if self.space_screen.living_checkbox {
                                include_purposes.insert(AreaType::Living);
                            }
                            if self.space_screen.military_checkbox {
                                include_purposes.insert(AreaType::Military);
                            }
                            if self.space_screen.party_checkbox {
                                include_purposes.insert(AreaType::Party);
                            }
                            if self.space_screen.science_checkbox {
                                include_purposes.insert(AreaType::Science);
                            }
                            if self.space_screen.industrial_checkbox {
                                include_purposes.insert(AreaType::Industrial);
                            }
                            let mut result: Vec<(Entity, String)> = Vec::new();
                            for room in rooms
                                .iter()
                                .filter(
                                    |(_, (atype, _, _, _) )| include_purposes.contains(atype)
                                ) {
                                    result.push((
                                        *(room.0),
                                        format!(
                                            "{}, вместимость: {} кв.м., свободно: {} кв.м.",
                                            room.1.0, room.1.1.0 / 100, room.1.2.0 / 100
                                        )));
                                };
                            result.sort_by(|(_, a), (_, b)|(*a).cmp(b));
                            for row in result {
                                if ui.button(row.1).clicked () {
                                    self.space_screen.selected_area = Some (row.0);
                                }
                            }
                        }
                    );
                    if let Some(entity) = self.space_screen.selected_area {
                        let mut room_contains: Vec<String> = Vec::new ();
                        let empty = Vec::new();
                        let people_in_room = people
                            .get(&entity)
                            .unwrap_or(&empty);
                        for human in people_in_room {
                            room_contains.push(format!(
                                "{} {}, занимает {} м.кв.",
                                human.0, human.1, human.2.0 / 100
                            ))
                        };
                        let empty2 = Vec::new();
                        let stationaries_in_room = stationaries
                            .get(&entity)
                            .unwrap_or(&empty2);

                        for stat in stationaries_in_room {
                            room_contains.push(format!(
                                "{}, занимает {} м.кв. Состояние объекта: {}",
                                stat.0, stat.1.0 / 100, stat.2
                            ));
                        };
                        room_contains.sort();
                        for row in room_contains {
                            cols[1].label (row);
                        }
                    };
                }
            );
            ui.separator ();
            if ui.button("Смена").clicked() {
                turn(&mut self.world, &mut self.resources);
            };
        });
    }


    fn demography_screen (
        &mut self,
        ctx: &CtxRef,
    ) {
        let people = people_by_profession(&mut self.world);
        CentralPanel::default().show(ctx, |ui| {
            let mut rows: Vec<String> = Vec::new();
            for ((prof, tier), count) in people {
                rows.push(format!("{} {} - {} чел", prof, tier, count));
            };
            rows.sort();
            for row in rows {
                 if ui.button(
                    row
                 ).clicked () {
                 }
            }
            ui.separator();
            if ui.button("Смена").clicked() {
                turn(&mut self.world, &mut self.resources);
            };
        });
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
