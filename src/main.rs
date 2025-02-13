use eframe::egui;
use std::{sync::{Arc, Mutex}, thread, time::Duration};
mod net;

pub use crate::net::network;
pub use crate::net::send_score;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "ClickyCrab",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    score: Arc<Mutex<i32>>,
    x: i32,
    upgd_cost: i32,
    proc: i32,
    autoclicker_running: Arc<Mutex<bool>>,
}

impl Default for MyApp {



    fn default() -> Self {
        Self {
            score: Arc::new(Mutex::new(0)),
            x: 1,
            upgd_cost: 10,
            proc: 0,
            autoclicker_running: Arc::new(Mutex::new(false)),
        }
    }
}


impl MyApp {

    fn arc_to_i32(&self) -> i32 {

    let guard = self.score.lock().unwrap();

    *guard
    }


    fn update_time(&mut self) {
        let score_arc = Arc::clone(&self.score);
        let x = self.x;
        let upgd_cost = self.upgd_cost;
        let tx = network();

        thread::spawn(move || {
            loop {
                println!("Send help send help send help Yasmina's holding me hostageeeeee");
                thread::sleep(Duration::from_secs(1));
                //network();
                // let tx = network();
                let current_score = {
                    let guard = score_arc.lock().unwrap();
                    *guard
                };

                send_score(tx.clone(), x, upgd_cost, current_score);
            }
        });
    }
}





impl eframe::App for MyApp {



    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.arc_to_i32();
        self.update_time();


        egui::CentralPanel::default().show(ctx, |ui| {
            // Clone the style from the Arc and modify it
            let mut style = (*ctx.style()).clone();
            style.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::proportional(24.0),
            );
            ctx.set_style(style);

            // Display the score
            ui.heading(format!("Score: {}", *self.score.lock().unwrap()));
            ui.heading(format!("Upgrade cost: {}", self.upgd_cost));
            ui.heading(format!("Autoclicker cost: {}", self.upgd_cost));

            // Buttons
            if ui.button("Click").clicked() {
                *self.score.lock().unwrap() += self.x;
            }

            if ui.button("Upgrade").clicked() {
                let mut score = self.score.lock().unwrap();
                if *score >= self.upgd_cost {
                    *score -= self.upgd_cost;
                    self.x += 1;
                    self.proc = *score * 40 / 100 + self.upgd_cost * 10 / 100;
                    self.upgd_cost += self.proc;
                }
            }

            if ui.button("Autoclick").clicked() {
                let mut autoclicker_running = self.autoclicker_running.lock().unwrap();
                if *self.score.lock().unwrap() >= self.upgd_cost && !*autoclicker_running {
                    *autoclicker_running = true;
                    let score_arc = Arc::clone(&self.score);
                    let x = self.x;
                    thread::spawn(move || {
                        loop {
                            {
                                let mut score = score_arc.lock().unwrap();
                                *score += x;
                            }
                            thread::sleep(Duration::from_secs(4));
                        }
                    });
                }
            }
        });
    }
}
