/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    tick_rate: f32,
    wish_speed: i32,
    strafes_per_jump: i32,
    starting_velocity: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            tick_rate: 60.0,
            wish_speed: 400,
            strafes_per_jump: 1,
            starting_velocity: 300.0,
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

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("strafe calculator");

            ui.horizontal(|ui| {
                ui.label("Tick rate: ");
                ui.add(egui::Slider::new(&mut self.tick_rate, 0.0..=200.0));
            });
        
            ui.horizontal(|ui| {
                ui.label("Starting speed: ");
                ui.add(egui::Slider::new(&mut self.starting_velocity, 300.0..=1000.0));
            });
            
            ui.horizontal(|ui| {
                ui.label("Strafe count: ");
                ui.add(egui::Slider::new(&mut self.strafes_per_jump, 1..=10));
            });

            ui.separator();

            let mut time = 0.0;
            let tick_interval = 1.0 / self.tick_rate;

            let mut strafe_path: PlotPoints;
            let mut strafe_length: f32 = 0.0;
            let mut speed: f32 = self.starting_velocity;
            let mut count = 0;

            let mut strafe_time = 0.75 / self.strafes_per_jump as f32;
            
            while time <= strafe_time {
                strafe_length = strafe_length + (30.0 / speed).asin().to_degrees();
                speed = ((30.0 * 30.0) + (speed * speed)).sqrt();
                time = time + tick_interval;
                count = count + 1;
            }

            let total_strafe_time = 0.75;
            speed = self.starting_velocity;
            time = 0.0;
            let mut current_angle: f32 = strafe_length * -0.5;
            ui.label(format!("{}", strafe_length));

            let mut switch_interval = 0.0;
            let mut direction = false;
            let mut points: Vec<[f64; 2]> = Vec::new();
            while time <= total_strafe_time {
                let mut angle_change = (30.0 / speed).asin().to_degrees() * 0.5;
                if switch_interval > strafe_time{
                    direction = !direction;
                    switch_interval = 0.;
                    angle_change *= 2.;
                }
                if direction {
                    current_angle = current_angle - angle_change;
                }
                else
                {
                    current_angle = current_angle + angle_change;
                }
                
                let mut amp = ((time / 2.) + 1.) as f64;
                points.push([current_angle.to_radians().sin() as f64 * amp, current_angle.to_radians().cos() as f64 * amp]);
                if direction {
                    current_angle = current_angle - angle_change;
                }
                else
                {
                    current_angle = current_angle + angle_change;
                }
                speed = ((30.0 * 30.0) + (speed * speed)).sqrt();
                time = time + tick_interval;
                switch_interval = switch_interval + tick_interval;
            }
            
            use egui_plot::{Line, Plot, PlotPoints};
            let sin: PlotPoints = PlotPoints::from(points);
            let line = Line::new(sin);
            Plot::new("my_plot").view_aspect(1.0).width(500.0).show_x(true).show_y(true).include_x(1.0).include_x(-1.0).include_y(1.0).include_y(-1.0)
            .label_formatter(|name, value| {
                if !name.is_empty() {
                    format!("{}: {:.*}%", name, 1, value.y)
                } else {
                    "".to_owned()
                }
            })
            .show(ui, |plot_ui| plot_ui.line(line));
            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
        // 0.75 is a full jump
        
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
