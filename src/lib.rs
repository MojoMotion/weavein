#![warn(clippy::all, rust_2018_idioms)]

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    eframe::start_web(canvas_id, Box::new(|cc| Box::new(Main::new(cc))))
}


use eframe::{ CreationContext, App, Storage, Frame,
    egui::{Context, CentralPanel, SidePanel, TopBottomPanel, TextEdit, Ui}, epaint::{TextureHandle, ColorImage},
};

pub struct Main {
    counter: i32,
    code: String,
    texture: Option<TextureHandle>,
}

impl Default for Main {
    fn default() -> Self {
        Main {
            counter: 0,
            code: "Here is some text !".into(),
            texture: None,
        }
    }
}

impl Main {
    #[allow(clippy::new_without_default)]
    #[allow(dead_code)]
    pub fn new(cc: &CreationContext<'_>) -> Main {
        let code =
            if let Some(storage) = cc.storage {
                eframe::get_value(storage, "code").unwrap_or_default()
            } else {
                Main::default().code
            }
        ;

        Main {
            code,
            ..Default::default()
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        let texture: &TextureHandle = self.texture.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture("my-image", ColorImage::example())
        });

        // Show the image:
        ui.image(texture, texture.size_vec2());
    }
}

impl App for Main {
    fn update(&mut self,
              ctx: &Context,
              _frame: &mut Frame)
    {
        SidePanel::left("app_bar")
            .min_width(2.0)
            .show(ctx, |ui| {
            ui.label("app_bar");

            ui.set_width(2.0);
            if ui.button("-").clicked() {
                self.counter -= 1;
            }

            ui.label(self.counter.to_string());

            if ui.button("+").clicked() {
                self.counter += 1;
            }

            self.ui(ui);
            //if ui.add(egui::ImageButton::new(texture, img_size)).clicked() {
            //    *boolean = !*boolean;
            //}
        });

        TopBottomPanel::top("head_bar").show(ctx, |ui| {
            ui.label("head_bar");
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");

            let text_block = TextEdit::multiline(&mut self.code);
            ui.add(text_block);

            //text = SyncSender::from(_)
            if ui.button("SAVE").clicked() {
                ctx.memory();
                ctx.data();

            }
        });

        ctx.request_repaint();
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, "code", &self.code)
    }
}
