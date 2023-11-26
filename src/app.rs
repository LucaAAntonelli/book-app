// use crate::db::all_books;
use chrono::NaiveDate;
use egui_extras::{Column, DatePickerButton, TableBuilder};
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
#[derive(serde::Deserialize, serde::Serialize)]
pub struct BookFromTable {
    title: String,
    authors: Vec<String>,
    num_pages: u64,
    #[serde(with = "naive_date_format")]
    acquisition_date: NaiveDate,
    #[serde(with = "optional_naive_date_format")]
    start_date: Option<NaiveDate>,
    #[serde(with = "optional_naive_date_format")]
    end_date: Option<NaiveDate>,
    price_ebook: f32,
    price_paperback: f32,
}

mod naive_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize};

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = date.format("%Y-%m-%d").to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
    }
}

mod optional_naive_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize};

    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match date {
            Some(d) => {
                let s = d.format("%Y-%m-%d").to_string();
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(date_str) => NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

pub struct TemplateApp {
    // Example stuff:
    query_str: String,
    value: f32,
    db_connection: Pool<sqlx::Postgres>,
    table: Option<Vec<BookFromTable>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let database_url = "postgres://postgres:mysecretpassword@localhost/postgres";
        let pool = rt
            .block_on(async {
                PgPoolOptions::new()
                    .max_connections(5)
                    .connect(database_url)
                    .await
            })
            .unwrap();

        Self {
            // Example stuff:
            query_str: "".to_owned(),
            value: 2.3,
            db_connection: pool,
            table: Option::None,
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

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Enter Book:");
                ui.text_edit_singleline(&mut self.query_str);
                if ui.button("Search").clicked() {
                    println!("Searching for {}...", self.query_str);
                }
            });

            ui.heading("List of Read Books");

            ui.vertical(|ui| {
                let text_height = egui::TextStyle::Body.resolve(ui.style()).size;
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .column(Column::initial(100.0).range(40.0..=300.0))
                    .column(Column::initial(100.0).at_least(40.0).clip(true))
                    .column(Column::initial(100.0))
                    .column(Column::initial(100.0))
                    .column(Column::initial(100.0))
                    .column(Column::initial(100.0))
                    .column(Column::initial(100.0))
                    .min_scrolled_height(0.0);

                table
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Title");
                        });
                        header.col(|ui| {
                            ui.strong("Authors");
                        });
                        header.col(|ui| {
                            ui.strong("Number of Pages");
                        });
                        header.col(|ui| {
                            ui.strong("Acquisition Date");
                        });
                        header.col(|ui| {
                            ui.strong("Start Date");
                        });
                        header.col(|ui| {
                            ui.strong("End Date");
                        });
                        header.col(|ui| {
                            ui.strong("Price eBook");
                        });
                        header.col(|ui| {
                            ui.strong("Price Paperback");
                        });
                    })
                    .body(|body| {
                        body.rows(text_height, 10, |row_index, mut row| {
                            row.col(|ui| {
                                ui.label(row_index.to_string());
                            });
                            row.col(|ui| {
                                ui.label("Column 1");
                            });
                            // row.col(|ui| {
                            //     if row_index % 2 == 0 {
                            //         let mut date = NaiveDate::from_ymd_opt(1070, 1, 1).unwrap();
                            //         ui.add(DatePickerButton::new(&mut date));
                            //     }
                            //     ui.label("Column 2");
                            // });
                            row.col(|ui| {
                                ui.add(
                                    egui::Label::new("Thousands of rows of even height")
                                        .wrap(false),
                                );
                            });
                        })
                    });
            });
        });
    }
}
