use std::sync::mpsc;
use eframe::{egui, epi};
use legion::*;
use std::collections::HashMap;

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

pub fn draw_ui(
    world: &mut World,
    ecs_resources: &mut Resources,
    texture_map: &mut HashMap<String, egui::TextureId>,
    resource_loaders: &mut HashMap<String, mpsc::Receiver<Vec<u8>>>,
    ctx: &egui::CtxRef,
    frame: &mut epi::Frame<'_>,
) {
    egui::SidePanel::left("left_panel", 80.0).show(ctx, |ui| {
        let button_txtr_size = [64.0, 64.0];
        fetch_resource_with_check(
            resource_loaders,
            texture_map,
            "assets/resources.png".to_string(),
            frame,
        );
        fetch_resource_with_check(
            resource_loaders,
            texture_map,
            "assets/demography.png".to_string(),
            frame,
        );
        fetch_resource_with_check(
            resource_loaders,
            texture_map,
            "assets/space.png".to_string(),
            frame,
        );
        fetch_resource_with_check(
            resource_loaders,
            texture_map,
            "assets/tasks.png".to_string(),
            frame,
        );
        decode_textures(resource_loaders, texture_map, frame);

        if ui
            .add(egui::ImageButton::new(
                get_texture_id(
                    texture_map,
                    "assets/resources.png".to_string(),
                ),
                button_txtr_size,
            )).on_hover_text("Ресурсы")
            .clicked()
        {
        }

        if ui
            .add(egui::ImageButton::new(
                get_texture_id(
                    texture_map,
                    "assets/demography.png".to_string(),
                ),
                button_txtr_size,
            )).on_hover_text("Демография")
            .clicked()
        {
        }

        if ui
            .add(egui::ImageButton::new(
                get_texture_id(
                    texture_map,
                    "assets/space.png".to_string(),
                ),
                button_txtr_size,
            )).on_hover_text("Пространство")
            .clicked()
        {
        }
        if ui
            .add(egui::ImageButton::new(
                get_texture_id(
                    texture_map,
                    "assets/tasks.png".to_string(),
                ),
                button_txtr_size,
            ))
            .on_hover_text("Производство")
            .clicked()
        {
        }
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("Смена").clicked() {
            turn(world, ecs_resources);
        }
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
    });
}
