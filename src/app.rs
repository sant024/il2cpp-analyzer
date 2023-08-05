use crate::helper::DataView;
use eframe::{egui, emath::Align};

use std::time::Duration;

// initialize once

// #[derive(Default)]
pub struct App {
    name: String,
    age: u32,
    testBool: bool,
    dv: DataView,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, dv: DataView) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        //Self::default()

        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            testBool: false,
            dv: dv,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("my_panel").show(ctx, |ui| {
            ui.label("Il2CppGlobalMetadataHeader Version: 21 ");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::left_to_right(Align::Center).with_cross_justify(true),
                |ui| {
                    ui.vertical(|ui| {
                        ui.heading("\tView");

                        egui::ScrollArea::vertical()
                            .id_source("image_scroll_area")
                            .show(ui, |ui| {
                                for image in &self.dv.image {
                                    ui.selectable_label(self.testBool, image);
                                }
                            });
                    });

                    ui.vertical(|ui| {
                        ui.heading("\tInspect");
                        egui::ScrollArea::vertical()
                            .id_source("class_scroll_area")
                            .show(ui, |ui| {
                                for (i, class) in self.dv.class_dump.iter().enumerate() {
                                    //for class in &self.dv.class_dump {

                                    ui.push_id(i, |ui| {
                                        // conflict class name
                                        ui.collapsing(&class.name, |ui| {
                                            for field in &class.fields {
                                                ui.label(&*field);
                                            }

                                            for method in &class.methods {
                                                ui.selectable_label(self.testBool, &*method);
                                            }
                                        });
                                    });
                                }

                                // Test
                                ui.collapsing("Niantic.Holoholo.UI.Transition", |ui| {
                                    // namespace
                                    ui.collapsing("ScreenWipeGuiController", |ui| {
                                        // class
                                        ui.label("private  unk targetTransform");
                                        ui.selectable_label(
                                            self.testBool,
                                            "public static Method(); // Here ",
                                        );
                                    });
                                });

                                ui.collapsing("Niantic.Holoholo.UI.Transitionsdsd", |ui| {
                                    // namespace
                                    ui.collapsing("ScreenWipeGuiControsdsdller", |ui| {
                                        // class
                                        ui.label("private  unk targetTrsdsdansform");
                                        ui.selectable_label(
                                            self.testBool,
                                            "public staic Method(); // dsdH5 ",
                                        );
                                    });
                                });
                            });
                    })
                },
            );
        });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     ui.heading("My egui Application");
        //     ui.horizontal(|ui| {
        //         let name_label = ui.label("Your name: ");
        //         ui.text_edit_singleline(&mut self.name)
        //             .labelled_by(name_label.id);
        //     });
        //     ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
        //     if ui.button("Click each year").clicked() {
        //         self.age += 1;
        //     }

        //     // test
        //     ui.horizontal(|ui| {
        //         let name_label = ui.label("dees name: ");
        //         ui.text_edit_singleline(&mut self.name)
        //             .labelled_by(name_label.id);
        //         ui.button("set")
        //     });

        //     ui.label(format!("Hello '{}', age {}", self.name, self.age));

        //     ui.collapsing("DockArea Options", |ui| {
        //         ui.checkbox(&mut self.testBool, "Show close buttons");
        //         ui.checkbox(&mut self.testBool, "Show add buttons");
        //         ui.checkbox(&mut self.testBool, "Draggable tabs");
        //         ui.checkbox(&mut self.testBool, "Show tab name on hover");
        //     });

        //     ui.collapsing("Border", |ui| {
        //         egui::Grid::new("border").show(ui, |ui| {
        //             ui.label("Width:");
        //             ui.add(egui::Slider::new(&mut self.age, 1..=50));
        //             ui.end_row();

        //             ui.label("Color:");
        //             ui.checkbox(&mut self.testBool, "Show close buttons");
        //             ui.end_row();

        //             ui.text_edit_multiline(&mut self.name);
        //         })
        //     });

        // });
    }
}
