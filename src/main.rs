#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod trainer;

use eframe::egui::{
    self, Color32, Stroke, Style, Theme, global_theme_preference_buttons, style::Selection,
};
use windows_sys::{Win32::UI::WindowsAndMessaging::*, core::*};

fn main() -> eframe::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([350.0, 500.0])
            .with_resizable(false)
            .with_maximized(false)
            .with_maximize_button(false),
        ..Default::default()
    };
    eframe::run_native(
        "PVZ +4 Trainer by un4ckn0wl3z",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

fn setup_custom_style(ctx: &egui::Context) {
    ctx.style_mut_of(Theme::Light, use_light_green_accent);
    ctx.style_mut_of(Theme::Dark, use_dark_purple_accent);
}

fn use_light_green_accent(style: &mut Style) {
    style.visuals.hyperlink_color = Color32::from_rgb(18, 180, 85);
    style.visuals.text_cursor.stroke.color = Color32::from_rgb(28, 92, 48);
    style.visuals.selection = Selection {
        bg_fill: Color32::from_rgb(157, 218, 169),
        stroke: Stroke::new(1.0, Color32::from_rgb(28, 92, 48)),
    };
}

fn use_dark_purple_accent(style: &mut Style) {
    style.visuals.hyperlink_color = Color32::from_rgb(202, 135, 227);
    style.visuals.text_cursor.stroke.color = Color32::from_rgb(234, 208, 244);
    style.visuals.selection = Selection {
        bg_fill: Color32::from_rgb(105, 67, 119),
        stroke: Stroke::new(1.0, Color32::from_rgb(234, 208, 244)),
    };
}

struct MyApp {
    trainer_states: trainer::TrainerStates,
    no_cool_down_toggle: bool,
    auto_collect_sun_toggle: bool,
    coin_value: u32,
    sun_value: u32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_style(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx); // Needed for the "Widget Gallery" demo
        Self {
            trainer_states: trainer::TrainerStates::new(),
            no_cool_down_toggle: false,
            auto_collect_sun_toggle: false,
            coin_value: 100,
            sun_value: 100,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Plants vs Zombies +4 Trainer");
            ui.label("Simple +4 Trainer for Plants vs Zombies, developed in Rust.");
            global_theme_preference_buttons(ui);
            ui.image(egui::include_image!("../assets/pvz.png"))
                .on_hover_text_at_pointer("Plants vs Zombies +4 Trainer");
            ui.separator();

            if ui
                .add(egui::Slider::new(&mut self.sun_value, 0..=99999).text("🌻Sun Value"))
                .changed()
            {
                match self.trainer_states.edit_sun_value(self.sun_value) {
                    Ok(_) => {}
                    Err(_) => unsafe {
                        MessageBoxW(
                            0 as _,
                            w!("Something wrong"),
                            w!("Trainer Error"),
                            MB_ICONERROR,
                        );
                        panic!("{}", "Something wrong".to_string());
                    },
                }
            }

            if ui
                .add(egui::Slider::new(&mut self.coin_value, 0..=99999).text("💵Coin Value"))
                .changed()
            {
                match self.trainer_states.edit_coin_value(self.coin_value) {
                    Ok(_) => {}
                    Err(_) => unsafe {
                        MessageBoxW(
                            0 as _,
                            w!("Something wrong"),
                            w!("Trainer Error"),
                            MB_ICONERROR,
                        );
                        panic!("{}", "Something wrong".to_string());
                    },
                }
            }

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.no_cool_down_toggle, "Disable Cool Down");
                if self.no_cool_down_toggle {
                    match self.trainer_states.no_cool_down(self.no_cool_down_toggle) {
                        Ok(_) => {}
                        Err(_) => unsafe {
                            MessageBoxW(
                                0 as _,
                                w!("Something wrong"),
                                w!("Trainer Error"),
                                MB_ICONERROR,
                            );
                            panic!("{}", "Something wrong".to_string());
                        },
                    }
                } else {
                    match self.trainer_states.no_cool_down(self.no_cool_down_toggle) {
                        Ok(_) => {}
                        Err(_) => unsafe {
                            MessageBoxW(
                                0 as _,
                                w!("Something wrong"),
                                w!("Trainer Error"),
                                MB_ICONERROR,
                            );
                            panic!("{}", "Something wrong".to_string());
                        },
                    }
                }

                // ---

                ui.checkbox(&mut self.auto_collect_sun_toggle, "Auto Sun Collecting");
                if self.auto_collect_sun_toggle {
                    match self
                        .trainer_states
                        .auto_collect_sun(self.auto_collect_sun_toggle)
                    {
                        Ok(_) => {}
                        Err(_) => unsafe {
                            MessageBoxW(
                                0 as _,
                                w!("Something wrong"),
                                w!("Trainer Error"),
                                MB_ICONERROR,
                            );
                            panic!("{}", "Something wrong".to_string());
                        },
                    }
                } else {
                    match self
                        .trainer_states
                        .auto_collect_sun(self.auto_collect_sun_toggle)
                    {
                        Ok(_) => {}
                        Err(_) => unsafe {
                            MessageBoxW(
                                0 as _,
                                w!("Something wrong"),
                                w!("Trainer Error"),
                                MB_ICONERROR,
                            );
                            panic!("{}", "Something wrong".to_string());
                        },
                    }
                }
            });
        });
        egui::TopBottomPanel::bottom("footer")
            .resizable(false) // Prevent resizing the footer
            .min_height(30.0) // Set a minimum height for the footer
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label("Developer: ");
                    ui.hyperlink_to("un4ckn0wl3z", "https://github.com/un4ckn0wl3z");
                    ui.label("Visit: ");
                    ui.hyperlink_to("eavesdropper.dev", "https://eavesdropper.dev/");
                });
            });
    }
}
