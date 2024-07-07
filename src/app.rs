use std::sync::{Arc, Mutex};

use egui::Ui;
use egui_extras::{Column, TableBuilder};
use ::goodreads_api::goodreads_api::GoodreadsBook;
use tokio::runtime::Runtime;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String, // search box label
    #[serde(skip)]
    books: Arc<Mutex<Vec<GoodreadsBook>>>, // field to allow asynchronous call to Goodreads API
    #[serde(skip)]
    search_button_clicked: bool, 
    #[serde(skip)]
    rt: tokio::runtime::Runtime,
    #[serde(skip)]
    search_in_progress: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            books: Arc::new(Mutex::new(vec![])),
            search_button_clicked: false,
            rt: Runtime::new().expect("Error creating runtime"),
            search_in_progress: false,
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

            // Search box for book queries
            ui.horizontal(|ui| { 
                ui.label("Enter Query: ");
                if ui.text_edit_singleline(&mut self.label).lost_focus() || ui.button("Search").clicked() {
                    // Permanently set self.search_button_clicked to true, keeps spinner active
                    self.search_button_clicked = true;
                    self.search_in_progress = true;
                }
                
            });

            // Table displaying query results
            ui.vertical(|ui| {
                let result_clone = Arc::clone(&self.books);
                let handle = self.rt.handle().clone();
                let label = self.label.clone();
                if self.search_button_clicked {
                    // Empty out books vector -> Ensures spinner display after consecutive querying
                    self.books.lock().unwrap().clear(); 
                    // Spawn tokio asynchronous task handler to call Goodreads API without blocking the main thread
                    handle.spawn(async move {
                        let res = GoodreadsBook::search(label.as_str()).await;
                        let mut result = result_clone.lock().unwrap();
                        *result = res;
                    });
                    // Reset search button state
                    self.search_button_clicked = false;
                }
                // Display a spinner while the API call is awaited
                if self.search_in_progress {
                    // Still waiting on async task to finish
                    ui.spinner();
                }
                // Once the vector is filled and unlocked, display it in a table
                if !self.books.lock().unwrap().is_empty() {
                    table_ui(ui, self.books.lock().unwrap().clone());
                    self.search_in_progress = false;
                }
            });
    });
}
}



fn table_ui(ui: &mut Ui, books: Vec<GoodreadsBook>) {
    TableBuilder::new(ui)
        .columns(Column::auto().resizable(true), 5)
        .sense(egui::Sense::click()) // Add sensing capabilities for each row in the table
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
            // Iterate over book vector, add row for each
            for book in books {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(book.title());
                    });
                    row.col(|ui| {
                        ui.label(book.authors().join(",")); // For multiple authors, join them to single string
                    });
                    row.col(|ui| {
                        ui.label(book.pages().to_string());
                    });
                    row.col(|ui| {
                        ui.label(book.series().unwrap_or_else(|| "None".to_string()));
                    });
                    row.col(|ui| {
                        ui.label(match book.index() {
                            None => "None".to_string(),
                            Some(f) => f.to_string(),
                        });
                    });
                    // For now, simply print selected book based on which column is clicked
                    if row.response().clicked() {
                        println!("{}", book);
                    }
                });
            }
        });
}