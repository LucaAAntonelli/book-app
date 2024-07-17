use std::{env, sync::{Arc, Mutex}};
use dotenv::dotenv;
use egui::{Ui, Widget};
use egui_extras::{Column, TableBuilder};
use ::goodreads_api::goodreads_api::GoodreadsBook;
use tokio::runtime::Runtime;
use log::{info, warn, error};

use crate::db::{self, DataBaseConnection};

#[derive(PartialEq)]
enum Panels {
    QueryGoodreads,
    UpdateDatabase,
    VisualizeData
}

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
    #[serde(skip)]
    current_panel: Panels,
    //#[serde(skip)]
    //database_connection: Arc<Mutex<db::DataBaseConnection>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        dotenv().ok();
        info!("Loading database URI from .env");
        let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
        info!("Successfully loaded URI: {db_uri}");
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        //let database_connection = rt.block_on(async { DataBaseConnection::new(&db_uri).await.unwrap() });
        info!("Connected to database");
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            books: Arc::new(Mutex::new(vec![])),
            search_button_clicked: false,
            rt, 
            search_in_progress: false,
            current_panel: Panels::QueryGoodreads,
           // database_connection: Arc::new(Mutex::new(database_connection)),
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

        // Load image loaders
        egui_extras::install_image_loaders(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_panel, Panels::QueryGoodreads, "Query Goodreads");
                ui.selectable_value(&mut self.current_panel, Panels::UpdateDatabase, "Update Database");
                ui.selectable_value(&mut self.current_panel, Panels::VisualizeData, "Visualize Data");
            });
        });

        match self.current_panel {
            Panels::QueryGoodreads => {
                egui::CentralPanel::default().show(ctx, |ui| {
                // The central panel the region left after adding TopPanel's and SidePanel's
                ui.heading("Book Tracker");

                // Search box for book queries
                ui.horizontal(|ui| { 
                    ui.label("Enter Query: ");
                    if ui.text_edit_singleline(&mut self.label).lost_focus() || ui.button("Search").clicked() {
                        // Permanently set self.search_button_clicked to true, keeps spinner active
                        info!("Detected search button click");
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
                        info!("Spawning tokio handle to call Goodreads API");
                        handle.spawn(async move {
                            info!("Spawned handle, calling Goodreads API");
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
                        info!("API call finished, rendering table...");
                        table_ui(ui, self.books.lock().unwrap().clone());
                        self.search_in_progress = false;
                    }
                });
            });
            },
            Panels::UpdateDatabase => {

            },
            Panels::VisualizeData => {

            }
        }

        
    }
}



fn table_ui( ui: &mut Ui, books: Vec<GoodreadsBook>) {
    TableBuilder::new(ui)
        .columns(Column::auto().resizable(true).at_least(40.0).at_most(70.0), 6)
        .sense(egui::Sense::click()) // Add sensing capabilities for each row in the table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Cover");
            });
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
                body.row(50.0, |mut row| {
                    row.col(|ui| {
                        egui::widgets::Image::new(book.cover_image().expect("No URL found"))
                            .fit_to_original_size(1 as f32)
                            .max_width(40.0)
                            .ui(ui);
                    });
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
                        info!("Book has been clicked, sending request to SQL database...");
                        println!("{}", book);
                        db::assert_send_book();
                        //tokio::task::spawn(async move {db_connection.lock().unwrap().insert_owned_book(book).await});
                        info!("Request sent");

                    }
                });
            }
        });
}