use chrono::{Date, Utc};
use eframe::{HardwareAcceleration, NativeOptions, Renderer, Theme};
use egui::containers::Frame;
use egui::{Id, Order, Pos2, Vec2};
use egui_extras::DatePickerPopup;

fn main() {
    let options = // eframe::NativeOptions::default();
    NativeOptions {
            always_on_top: false,
            maximized: false,
            decorated: true,
            drag_and_drop_support: true,
            icon_data: None,
            initial_window_pos: Some(Pos2{x: 1580.0, y: 35.0}),
            initial_window_size: Some(Vec2 {x: 200.0, y: 175.0}),
            min_window_size: Some(Vec2 {x: 200.0, y: 175.0}),
            max_window_size: Some(Vec2 {x: 340.0, y: 100.0}),
            resizable: true,
            transparent: false,
            vsync: true,
            multisampling: 0,
            depth_buffer: 0,
            stencil_buffer: 0,
            hardware_acceleration: HardwareAcceleration::Preferred,
            renderer: Renderer::default(),
            follow_system_theme: cfg!(target_os = "macos") || cfg!(target_os = "windows"),
            default_theme: Theme::Dark,
    };

    eframe::run_native("cal", options, Box::new(|_| Box::new(MyApp::default())));
}

struct MyApp {
    selection: Date<Utc>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            selection: Utc::today(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Area::new("cal")
            .order(Order::Foreground)
            .fixed_pos(Pos2 { x: 0.0, y: 0.0 })
            .show(ctx, |ui| {
                // ui.set_min_width(333.0); // changing this adjusts the calendar control width
                // ui.set_max_width(333.0);

                let frame = Frame::popup(ui.style());
                frame.show(ui, |ui| {
                    DatePickerPopup {
                        selection: &mut self.selection,
                        button_id: Id::new(1),
                        combo_boxes: true,
                        arrows: true,
                        calendar: true,
                        calendar_week: true,
                    }
                    .draw(ui);
                });
            });
    }
}
