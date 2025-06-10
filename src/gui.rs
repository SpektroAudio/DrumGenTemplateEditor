#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use crate::drumgen::*;
use egui::*;
use egui::special_emojis::GITHUB;
use egui::style::HandleShape;

use simplelog::*;
use log::{LevelFilter};
use std::fs::File;

/*
 
   __  __      _         ___ _   _ ___ 
  |  \/  |__ _(_)_ _    / __| | | |_ _|
  | |\/| / _` | | ' \  | (_ | |_| || | 
  |_|  |_\__,_|_|_||_|  \___|\___/|___|
                                       
 
*/

pub fn main_gui() -> Result<(), eframe::Error> {

    #[cfg(feature="log")]
    let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("editor.log").unwrap());
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(1100.0, 600.0))
            ,
        ..Default::default()
    };

    eframe::run_native(
        "NGEN – DrumGen Template Editor",
        options,
        Box::new(|_cc| {
            Box::<DrumgenEditor>::default()
        }),
    )
}

/*
 
     _               ___       __ _      _ _   _          
    /_\  _ __ _ __  |   \ ___ / _(_)_ _ (_) |_(_)___ _ _  
   / _ \| '_ \ '_ \ | |) / -_)  _| | ' \| |  _| / _ \ ' \ 
  /_/ \_\ .__/ .__/ |___/\___|_| |_|_||_|_|\__|_\___/_||_|
        |_|  |_|                                          
 
*/

pub struct DrumgenEditor {
    template: DrumGen,
    name: String,
    active_part: usize,
    value_display: String,
    output: Vec<u8>,
    random_min: u8,
    random_max: u8,
    random_probability: u8,
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

impl DrumgenEditor {
    fn get_filename(&self) -> String {
        format!("{}.hex", self.name).to_uppercase()
    }
}

impl Default for DrumgenEditor {
    fn default() -> Self {
        Self {
            name: "DGT_0001".to_owned(),
            template: DrumGen::new(),
            active_part: 0,
            value_display: format!("-"),
            output: vec![0; 192],
            random_min: 0,
            random_max: 10,
            random_probability: 100,
            dropped_files: Vec::new(),
            picked_path: None,
        }
    }
}

impl eframe::App for DrumgenEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            // Set UI selection background fill color 
            ui.visuals_mut().selection.bg_fill = egui::Color32::from_rgba_unmultiplied(115, 193, 173, 100);
            ui.visuals_mut().selection.stroke = egui::Stroke::new(1.0, egui::Color32::from_black_alpha(200));
            ui.visuals_mut().slider_trailing_fill = true;

            // Header
            ui.heading(egui::RichText::new("NGEN - DrumGen Template Editor").strong());
            ui.separator();

            // File Management
            ui.horizontal(|ui|{
                ui.spacing_mut().button_padding = egui::vec2(10.0, 10.0);
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.picked_path = Some(path.display().to_string());
                        self.template.parse_file(path.display().to_string());
                    }
                }
                if ui.button("Save").clicked() {
                    match self.picked_path {
                        Some(ref path) => {
                            self.template.save_file(path.to_string());
                        }
                        None => {
                            if let Some(path) = rfd::FileDialog::new().set_file_name(self.get_filename()).save_file() {
                                self.template.save_file(path.display().to_string());
                            }
                        }
                    }
                }
                if ui.button("Save As…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().set_file_name(self.get_filename()).save_file() {
                        self.template.save_file(path.display().to_string());
                    }
                }
                ui.add_space(20.00);
                ui.vertical(|ui| {
                    ui.label("Template Name");
                    ui.add_sized(egui::Vec2{x: 100.0, y: 10.0}, egui::TextEdit::singleline(&mut self.name).char_limit(8));                
                });
            });


            // Part Selector
            ui.separator();
            ui.heading(egui::RichText::new("PART SELECTOR"));
            ui.horizontal(|ui| {

                ui.spacing_mut().button_padding = egui::vec2(10.0, 10.0);
                
                if ui.add(egui::SelectableLabel::new(self.active_part == 0, "Part 1")).clicked(){
                    self.active_part = 0;
                };
                if ui.add(egui::SelectableLabel::new(self.active_part == 1, "Part 2")).clicked(){
                    self.active_part = 1;
                };
                if ui.add(egui::SelectableLabel::new(self.active_part == 2, "Part 3")).clicked(){
                    self.active_part = 2;
                };
                if ui.add(egui::SelectableLabel::new(self.active_part == 3, "Part 4")).clicked(){
                    self.active_part = 3;
                };
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    add_sequence_gui(ui, self,  0, 127);
                    ui.separator();
                    add_sequence_gui(ui, self, 1, 100);
                    ui.separator();
                    add_sequence_gui(ui, self, 2, 60);
                });
                
            });
            ui.separator();

            // Bottom Panel
            egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show_inside(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.horizontal(|ui| {
                        ui.hyperlink_to(
                            format!("{GITHUB} NGEN repo on GitHub"),
                            "https://github.com/SpektroAudio/NGEN-Resources",
                        );
                        ui.label(" | ");
                        ui.hyperlink_to(
                            format!("NGEN User Manual"),
                            "https://ngen.spektroaudio.com");
                    });
                });
            });

            // Left Panel
            egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(180.0)
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Shortcuts");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Num 1-8: Repeat every x");
                });
            });


            // Right Panel
            egui::CentralPanel::default()
                .show_inside(ui, |ui| {
                    ui.spacing_mut().slider_width = 40.0;
                    ui.label("Randomize Settings");
                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::Slider::new(&mut self.random_min, 0..=10)
                                    .text("Min")
                                    .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 })
                                );
                            ui.label(" | ");
                            ui.add(egui::Slider::new(&mut self.random_max, 0..=10)
                                .text("Max")
                                .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 })
                            );
                            ui.label(" | ");
                            ui.add(
                                egui::Slider::new(&mut self.random_probability, 0..=100)
                                .text("Prob").handle_shape(HandleShape::Rect { aspect_ratio: 0.5 })
                            );
                        });
                    });
            });

            // Right Panel
            egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(340.0)
            
            .width_range(200.0..=400.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Output Preview");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        if ui.button("Convert").clicked() {
                            self.output = self.template.convert();
                        }
                        let mut out_text: String = "".to_string();
                        for y in 0..12 {
                            for x in 0..16 {
                                let index = x + (y * 16);
                                let label_txt = format!("{:02X} ", self.output[index]);
                                out_text += &label_txt;
                            }
                            out_text += "\n";
                        }
                        ui.add(egui::TextEdit::multiline(&mut out_text).desired_width(300.0));
                });
                });
                
            });


            preview_files_being_dropped(ctx);

            // Collect dropped files:
            ctx.input(|i| {
                if !i.raw.dropped_files.is_empty() {
                    self.dropped_files = i.raw.dropped_files.clone();
                }
            });

            
            
        });
    }
}


fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}



pub fn add_sequence_gui(ui: &mut Ui, editor: &mut DrumgenEditor, layer: usize, velocity: u8, ) {
    ui.label(egui::RichText::new(format!("VELOCITY: {}", velocity)).strong());
    ui.horizontal(|ui| {
        ui.spacing_mut().slider_width = 50.0;

        egui::ScrollArea::horizontal()
            .scroll_bar_visibility(egui::containers::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
            .id_source(format!("sequence_{}", layer))
            .show(ui, |ui| {
            ui.vertical(|ui| {
                
                
                let button_size = egui::vec2(36.0, 20.0);
                ui.horizontal(|ui| {
                    let randomize_button = ui.add_sized(button_size.clone(), egui::Button::new("R")).on_hover_text("Randomize");
                    let clear_button = ui.add_sized(button_size.clone(), egui::Button::new("C")).on_hover_text("Clear");

                    if randomize_button.clicked() {
                        editor.template.parts[editor.active_part].sequence[layer].randomize(editor.random_min, editor.random_max, editor.random_probability);
                    }

                    if clear_button.clicked() {
                        editor.template.parts[editor.active_part].sequence[layer].clear();
                    }
                });
                ui.horizontal(|ui| {
                    let shift_left = ui.add_sized(button_size.clone(), egui::Button::new("<<")).on_hover_text("Shift left");
                    let shift_right = ui.add_sized(button_size.clone(), egui::Button::new(">>")).on_hover_text("Shift right");
                    if shift_left.clicked() {
                        editor.template.parts[editor.active_part].sequence[layer].shift(1);
                    }
                    if shift_right.clicked() {
                        editor.template.parts[editor.active_part].sequence[layer].shift(-1);
                    }
                });
                ui.horizontal(|ui| {
                    let minus_one = ui.add_sized(button_size.clone(), egui::Button::new("-1")).on_hover_text("Add -1");
                    let plus_one = ui.add_sized(button_size.clone(), egui::Button::new("+1")).on_hover_text("Add +1");
                    if minus_one.clicked() {
                        editor.template.parts[editor.active_part].sequence[layer].add(-1);
                    }
                    if plus_one.clicked() {
                        editor.template.parts[editor.active_part].sequence[layer].add(1);
                    }
                });
            });
            ui.add_space(20.0);
                let seq_len = editor.template.parts[editor.active_part].sequence[layer].steps.len();
                for i in 0..seq_len {
                    let slider = ui.add(egui::Slider::new(&mut editor.template.parts[editor.active_part].sequence[layer].steps[i], 0..=10)
                    .vertical()
                    .show_value(false).text(format!(" {}", i+1))
                    .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 })
                    .trailing_fill(true)
                    
                
                    );
                    if slider.hovered() {
                        editor.value_display = format!("Probability: {}%", editor.template.parts[editor.active_part].sequence[layer].steps[i] as f32 * 10.0);
                    };
                    if slider.drag_released() {
                        // Check if holding down the number 1
                        let mut skip_value = 0;
                        if ui.input(|ui| ui.key_down(egui::Key::Num1)) {
                            skip_value = 1;
                        }
                        if ui.input(|ui| ui.key_down(egui::Key::Num2)) {
                            skip_value = 2;
                        }
                        if ui.input(|ui| ui.key_down(egui::Key::Num3)) {
                            skip_value = 3;
                        }
                        if ui.input(|ui| ui.key_down(egui::Key::Num4)) {
                            skip_value = 4;
                        }
                        if ui.input(|ui| ui.key_down(egui::Key::Num5)) {
                            skip_value = 5;
                        }
                        if ui.input(|ui| ui.key_down(egui::Key::Num6)) {
                            skip_value = 6;
                        }
                        if ui.input(|ui| ui.key_down(egui::Key::Num7)) {
                            skip_value = 7;
                        }
                        if ui.input(|ui| ui.key_down(egui::Key::Num8)) {
                            skip_value = 8;
                        }
                        if skip_value > 0 {
                            let mut index = i;
                            for _ in i..editor.template.parts[editor.active_part].sequence[layer].steps.len() {
                                index += skip_value;
                                if index < editor.template.parts[editor.active_part].sequence[layer].steps.len() {
                                    editor.template.parts[editor.active_part].sequence[layer].steps[index] = editor.template.parts[editor.active_part].sequence[layer].steps[i];
                                }
                            }
                        }
                    };
                    if i % 4 == 3 && i < (seq_len - 1) {
                        ui.add_space(20.0f32)
                    }
                }
            });
    });
}