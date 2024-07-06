use std::sync::{Arc, Mutex};

use egui::{Spinner, Ui};
use egui_extras::{Column, TableBuilder};
use ::goodreads_api::goodreads_api::GoodreadsBook;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    #[serde(skip)]
    books: Vec<GoodreadsBook>,
    #[serde(skip)]
    search_button_clicked: bool,
    #[serde(skip)]
    rt: tokio::runtime::Runtime,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            books: vec![],
            search_button_clicked: false,
            rt: Runtime::new().expect("Error creating runtime"),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            

                
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Book Tracker");

            ui.horizontal(|ui| { 
                ui.label("Enter Query: ");
                ui.text_edit_singleline(&mut self.label);
                self.search_button_clicked = ui.button("Enter").clicked();
                
            });

            ui.vertical(|ui| {
                
                ui.add(Spinner::new());
                if self.search_button_clicked {

                    self.books = self.rt.block_on(async {
                        
                        GoodreadsBook::search(self.label.as_str()).await});
                    
                    for book in &self.books {
                        println!("{book}");
                    }
                }
                if !self.books.is_empty() {
                    table_ui(ui, self.books.clone());
                }
            });
    });
}
}

fn table_ui(ui: &mut Ui, books: Vec<GoodreadsBook>) {
    TableBuilder::new(ui)
            .columns(Column::auto().resizable(true), 5)
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Title");
                });
                header.col(|ui| {
                    ui.strong("Author(s)");
                });
                header.col(|ui| {
                    ui.strong("No. Pages");
                });
                header.col(|ui| {
                    ui.strong("Series");
                });
                header.col(|ui| {
                    ui.strong("Volume");
                });
            })
            .body(|mut body| {

                for book in books {
                    body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(book.title());
                    });
                    row.col(|ui| {
                        ui.label(book.authors().join(","));
                    });
                    row.col(|ui| {
                        ui.label(book.pages().to_string());
                    });
                    row.col(|ui| {
                        ui.label(book.series().unwrap_or("None".to_string()));
                    });
                    row.col(|ui| {
                        ui.label({match book.index() {
                            None => "None".to_string(),
                            Some(f) => f.to_string()
                        }});
                    });
                });}
            });
}