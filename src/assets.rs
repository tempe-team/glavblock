use std::sync::mpsc;
use image::io::Reader;
use std::collections::HashMap;
use eframe::{
    egui,
    epi,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use futures::executor::block_on;
#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, RequestMode, Response};
#[cfg(target_arch = "wasm32")]
use egui_web::http::{fetch_async};


// #[cfg(not(target_arch = "wasm32"))]
// use std::fs::File;
// #[cfg(not(target_arch = "wasm32"))]
// use std::io::BufReader;

pub struct Image {
    pub size: (usize, usize),
    pub pixels: Vec<egui::Color32>,
}

impl Image {
    fn decode(bytes: &[u8]) -> Option<Image> {
        use image::GenericImageView;
        let image = image::load_from_memory(bytes).ok()?;
        let image_buffer = image.to_rgba8();
        let size = (image.width() as usize, image.height() as usize);
        let pixels = image_buffer.into_vec();
        assert_eq!(size.0 * size.1 * 4, pixels.len());
        let pixels = pixels
            .chunks(4)
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        Some(Image { size, pixels })
    }
}

/// Скачиваем ресурсы
#[cfg(target_arch = "wasm32")]
pub fn fetch_resource(
    resource_loaders: &mut HashMap<String, mpsc::Receiver<Vec<u8>>>,
    path: String,
    frame: &mut epi::Frame<'_>,
) {
    let url = format!("/{}", path);
    let (sender, receiver) = std::sync::mpsc::channel();
    resource_loaders.insert(path, receiver);
    frame.http_fetch(
        epi::http::Request::get(url),
        move |response| {
            let bytes = match response {
                Ok (resp) => resp.bytes,
                Err (_) => Vec::new (),
            };
            sender.send(bytes).ok();
        }
    );
}

#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_resource(
    resource_loaders: &mut HashMap<String, mpsc::Receiver<Vec<u8>>>,
    path: String,
    _frame: &mut epi::Frame<'_>,
) {
    let img = Reader::open(
        path.clone()
    ).unwrap().decode().unwrap();
    let mut buffer = Vec::new();
    img.write_to(
        &mut buffer,
        image::ImageOutputFormat::Png,
    ).unwrap();
    let (sender, receiver) = std::sync::mpsc::channel();
    resource_loaders.insert(path, receiver);
    sender.send(buffer);
}

/// Поставить в очередь на скачку ресурс.
/// С проверкой на предмет того, скачивается ли он или возможжно уже скачан.
pub fn fetch_resource_with_check (
    resource_loaders: &mut HashMap<String, mpsc::Receiver<Vec<u8>>>,
    texture_map: &mut HashMap<String, egui::TextureId>,
    path: String,
    frame: &mut epi::Frame<'_>,
) {
    if resource_loaders.contains_key(&path) // скачивается
        || texture_map.contains_key(&path) // скачано
    {
        // Ничего не делать
    } else {
        fetch_resource(resource_loaders, path, frame)
    }
}

/// Храним текстуры на стороне egui(в frame), а сдвиги в текстурной карте - на стороне приложения.
pub fn decode_textures(
    resource_loaders: &mut HashMap<String, mpsc::Receiver<Vec<u8>>>,
    texture_map: &mut HashMap<String, egui::TextureId>,
    frame: &mut epi::Frame<'_>,
) {
    let mut to_delete = Vec::new();
    for (
        path,
        recv,
    ) in resource_loaders.iter() {
        if let Ok (raw) = recv.try_recv() {
            let image = Image::decode(
                raw.as_slice()
            ).unwrap();
            let texture_id = frame
                .tex_allocator()
                .alloc_srgba_premultiplied(
                    image.size,
                    &image.pixels,
                );
            texture_map.insert(path.clone(), texture_id);
            to_delete.push(path.clone());
        }
    };
    for k in to_delete.iter () {
        resource_loaders.remove(k);
    }
}

pub fn get_texture_id (
    texture_map: &mut HashMap<String, egui::TextureId>,
    path: String,
) -> egui::TextureId {
    match texture_map.get (&path) {
        None => Default::default (),
        Some (texture_id) => texture_id.clone(),
    }
}


pub fn load_all_resources(
    resource_loaders: &mut HashMap<String, mpsc::Receiver<Vec<u8>>>,
    texture_map: &mut HashMap<String, egui::TextureId>,
    frame: &mut epi::Frame<'_>,
) {
    let paths = vec![
        "assets/resources.png".to_string(),
        "assets/demography.png".to_string(),
        "assets/space.png".to_string(),
        "assets/tasks.png".to_string(),
        "assets/military.png".to_string(),
        "assets/party.png".to_string(),
        "assets/industrial.png".to_string(),
        "assets/science.png".to_string(),
        "assets/living.png".to_string(),
    ];

    for path in paths.iter() {
        fetch_resource_with_check(
            resource_loaders,
            texture_map,
            path.clone(),
            frame,
        );
    };
}
