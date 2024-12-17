use eframe::egui;
//use rand::Rng;
use std::{thread, time::Duration};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Clicker Game Thingy",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    score: i32,
    x: i32,
    upgd_cost: i32,
    proc: i32,
    constt: i32,
   // rand_number: i32
}

impl Default for MyApp {
    fn default() -> Self {
        //let mut rng = rand::thread_rng();
        //let rand_number = rng.gen_range(1..=50);
        Self { score: 0, x: 1, upgd_cost: 10, proc: 0, constt: 0}

    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Clone the style from the Arc and modify it
            let mut style = (*ctx.style()).clone();
            style.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::proportional(24.0),
            );
            ctx.set_style(style);

            // Display the score
            ui.heading(format!("Score: {}", self.score));
            ui.heading(format!("Upgrade cost: {}", self.upgd_cost));
            ui.heading(format!("Autoclicker cost: {}", self.upgd_cost));

            // Buttons
            if ui.button("Click").clicked() {
                self.score += self.x;
            }

            if ui.button("Upgrade").clicked() {
                if self.score >= self.upgd_cost {
                    self.score = self.score - self.upgd_cost;
                    self.x = self.x + 1;
                    self.proc = self.score * 40/100 + self.upgd_cost * 10/100;
                    self.upgd_cost = self.upgd_cost + self.proc;

                }
                

            }

            if ui.button("Autoclick").clicked() {
                if self.score >= self.upgd_cost {
                    while self.constt == self.constt {
                        thread::sleep(Duration::from_millis(4000));
                        self.score = self.score + self.x



                    }

                }

            }
        });
    }
}
