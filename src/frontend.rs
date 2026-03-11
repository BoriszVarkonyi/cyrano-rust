use eframe::egui;

pub struct MyApp {
    counter: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Counter Example");

            ui.label(format!("Counter: {}", self.counter));

            if ui.button("Increment").clicked() {
                self.counter += 1;
            }

        });

    }
}