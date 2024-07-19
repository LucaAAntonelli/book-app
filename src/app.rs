use std::{env, sync::Arc};
use dotenv::dotenv;
use egui::{Ui, Widget};
use egui_extras::{Column, DatePickerButton, TableBuilder};
use ::goodreads_api::goodreads_api::GoodreadsBook;
use log::{info, error};
use tokio::sync::Mutex;
use crate::db::{self, DataBaseConnection};

#[derive(PartialEq)]
enum Panels {
    QueryGoodreads,
    UpdateDatabase,
    VisualizeData
}


pub struct TemplateApp {
    // Example stuff:
    label: String, // search box label
    books: Arc<std::sync::Mutex<Vec<GoodreadsBook>>>, // field to allow asynchronous call to Goodreads API
    search_button_clicked: bool, 
    rt: tokio::runtime::Runtime,
    search_in_progress: bool,
    current_panel: Panels,
    database_connection: Arc<Mutex<db::DataBaseConnection>>,
    file_dialog: egui_file_dialog::FileDialog,
    selected_book: Option<GoodreadsBook>,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.


        dotenv().ok();
        info!("Loading database URI from .env");
        let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
        info!("Successfully loaded URI");
        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        let database_connection = rt.block_on(async { DataBaseConnection::new(&db_uri).await.unwrap() });
        info!("Connected to database");
        Self {
            // Example stuff:
            label: "".to_owned(),
            books: Arc::new(std::sync::Mutex::new(vec![])),
            search_button_clicked: false,
            rt, 
            search_in_progress: false,
            current_panel: Panels::QueryGoodreads,
            database_connection: Arc::new(Mutex::new(database_connection)),
            file_dialog: egui_file_dialog::FileDialog::new(),
            selected_book: None,
        }
    }
}

impl eframe::App for TemplateApp {
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
                    ui.vertical(|ui| {
                        if ui.text_edit_singleline(&mut self.label).lost_focus() || ui.button("Search").clicked() {
                            // Permanently set self.search_button_clicked to true, keeps spinner active
                            info!("Detected search button click");
                            self.search_button_clicked = true;
                            self.search_in_progress = true;
                        }
                        if ui.button("Import from Excel").clicked() {
                            info!("Opening file dialog");
                            self.file_dialog.select_file();

                        }
                    });
                    
                    if let Some(path) = self.file_dialog.update(ctx).selected() {
                        info!("File dialog selected a file");
                        println!("{}", path.to_str().unwrap());
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
                        self.table_ui(ui);
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


impl TemplateApp {


    fn table_ui(&mut self, ui: &mut Ui) {
        let books = self.books.lock().unwrap().clone();
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
                            ui.label(book.authors().join(", ")); // For multiple authors, join them to single string
                        });
                        row.col(|ui| {
                            ui.label(book.pages().to_string());
                        });
                        row.col(|ui| {
                            ui.label(match book.series_info() {
                                Some(series_info) => series_info.iter().map(|x| format!("{}", x.name())).collect::<Vec<_>>().join("\n\n"),
                                None => "".to_owned(),
                            });
                        });
                        row.col(|ui| {
                            ui.label(match book.series_info() {
                                Some(series_info) => series_info.iter().map(|x| format!("# {}", x.volume())).collect::<Vec<_>>().join("\n\n"),
                                None => "".to_owned(),
                            });
                        });
                        // For now, simply print selected book based on which column is clicked
                        if row.response().clicked() {
                            self.selected_book = Some(book);
                        }
                    });
                }
            });
        if let Some(book) = self.selected_book.clone() {
            info!("Book has been clicked, sending request to SQL database...");
            info!("{}", book);
            info!("Opening date picker widget");
            let mut selected_date = chrono::Utc::now().date_naive();
            ui.add(DatePickerButton::new(&mut selected_date));
            
            
            let db_connection_clone = Arc::clone(&self.database_connection);
            self.rt.spawn(async move {
                match db_connection_clone.lock().await.insert_owned_book(book).await {
                    Ok(_) => info!("Query sent successfully"),
                    Err(e) => error!("Could not send query: {e}")
                }
            });
        }
            
            self.selected_book = None; // Reset to trigger only once per selection
    }
}

