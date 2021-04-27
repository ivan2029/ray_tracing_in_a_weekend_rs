use crate::{
    app::scenes::{scene_creators, SceneCreators},
    raytracer::color::Color,
};

use eframe::{egui, epi};

use rayon::{ThreadPool, ThreadPoolBuilder};

use crossbeam::channel;

use std::time::Duration;

//
//
//
enum AppState {
    Setup,
    Rendering,
}

//
//
//
struct Chunk {
    x: usize,
    y: usize,
    duration: Duration,
    pixels: Vec<Color>,
}

//
//
//
pub struct App {
    //
    state: AppState,

    //
    selected_scene: &'static str,
    scene_creators: SceneCreators,

    //
    image_width: usize,
    image_height: usize,
    sample_count: usize,
    max_depth: usize,
    chunk_size: usize,

    //
    tex_id: Option<egui::TextureId>,
    pixels: Vec<egui::Color32>,

    //
    total_chunk_count: usize,
    chunks_received: usize,

    chunk_channel: Option<channel::Receiver<Chunk>>,
    thread_pool: Option<ThreadPool>,
}

impl App {
    fn update_when_setup(
        &mut self,
        ctx: &egui::CtxRef,
        frame: &mut epi::Frame<'_>,
    ) {
        //
        let mut render_clicked = false;

        //
        egui::SidePanel::left("setup_panel", 200.0).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                //
                egui::ComboBox::from_label("Select scene")
                    .selected_text(self.selected_scene)
                    .show_ui(ui, |ui| {
                        for (key, _) in &self.scene_creators {
                            ui.selectable_value(&mut self.selected_scene, *key, *key);
                        }
                    });

                //
                ui.separator();

                //
                egui::Grid::new("setup_grid").show(ui, |ui| {
                    ui.label("Image width");
                    ui.add(egui::Slider::new(&mut self.image_width, 10..=4000));
                    ui.end_row();

                    ui.label("Image height");
                    ui.add(egui::Slider::new(&mut self.image_height, 10..=4000));
                    ui.end_row();

                    ui.label("Samples per pixel");
                    ui.add(egui::Slider::new(&mut self.sample_count, 1..=1000));
                    ui.end_row();

                    ui.label("Max depths");
                    ui.add(egui::Slider::new(&mut self.max_depth, 1..=1000));
                    ui.end_row();

                    ui.label("Chunk size");
                    ui.add(egui::Slider::new(&mut self.chunk_size, 1..=128));
                    ui.end_row()
                });

                //
                ui.separator();

                //
                render_clicked = ui.button("Render").clicked();
            })
        });

        if let Some(tex_id) = self.tex_id {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.image(
                        tex_id,
                        egui::Vec2::new(self.image_width as f32, self.image_height as f32),
                    );
                })
            });
        }

        if render_clicked {
            self.start_render(frame);
        }
    }

    fn update_when_rendering(
        &mut self,
        ctx: &egui::CtxRef,
        frame: &mut epi::Frame<'_>,
    ) {
        //
        let next = self.chunk_channel.as_ref().map(|c| c.recv());
        if let Some(Ok(chunk)) = next {
            log::info!(
                "Got chunk: x = {:>4}, y = {:>4}, time {:?}",
                chunk.x,
                chunk.y,
                chunk.duration
            );

            self.chunks_received += 1;

            //
            for j in 0..self.chunk_size {
                let y = chunk.y * self.chunk_size + j;

                if y >= self.image_height {
                    continue;
                }

                for i in 0..self.chunk_size {
                    let x = chunk.x * self.chunk_size + i;

                    if x >= self.image_width {
                        continue;
                    }

                    let color = chunk.pixels[j * self.chunk_size + i].as_u8();

                    self.pixels[y * self.image_width + x] =
                        egui::Color32::from_rgb(color[0], color[1], color[2]);
                }
            }

            //
            self.tex_id = Some(
                frame
                    .tex_allocator()
                    .alloc_srgba_premultiplied((self.image_width, self.image_height), &self.pixels),
            );
        }

        //
        let mut stop_render_clicked = false;

        egui::SidePanel::left("render_control_panel", 200.0).show(ctx, |ui| {
            ui.label(format!(
                "Done {}%",
                self.chunks_received * 100 / self.total_chunk_count
            ));

            ui.separator();

            stop_render_clicked = ui.button("Stop render").clicked();
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                let tex_id = self.tex_id.clone().unwrap();
                ui.image(
                    tex_id,
                    egui::Vec2::new(self.image_width as f32, self.image_height as f32),
                );
            })
        });

        if stop_render_clicked {
            self.stop_render();
        }
    }

    fn start_render(
        &mut self,
        frame: &mut epi::Frame<'_>,
    ) {
        log::info!("starting render");

        self.state = AppState::Rendering;

        //
        self.pixels.resize(
            self.image_width * self.image_height,
            egui::Color32::from_rgb(32, 32, 32),
        );
        self.pixels.fill(egui::Color32::from_rgb(32, 32, 32));

        self.tex_id = Some(
            frame
                .tex_allocator()
                .alloc_srgba_premultiplied((self.image_width, self.image_height), &self.pixels),
        );

        //
        let (sender, receiver) = channel::bounded(1000);

        self.chunk_channel = Some(receiver);

        self.thread_pool = Some(ThreadPoolBuilder::new().build().unwrap());

        self.total_chunk_count = {
            let xs = (self.image_width + self.chunk_size - 1) / self.chunk_size;
            let ys = (self.image_height + self.chunk_size - 1) / self.chunk_size;
            xs * ys
        };
        self.chunks_received = 0;

        //
        let scene = self.scene_creators.get(self.selected_scene).unwrap()();

        //
        let image_width = self.image_width;
        let image_height = self.image_height;
        let sample_count = self.sample_count;
        let max_depth = self.max_depth;
        let chunk_size = self.chunk_size;

        self.thread_pool.as_ref().unwrap().spawn(move || {
            raytrace_task(
                sender,
                scene,
                image_width,
                image_height,
                sample_count,
                max_depth,
                chunk_size,
            )
        });
    }

    fn stop_render(&mut self) {
        log::info!("stopping render");

        drop(self.thread_pool.take());
        drop(self.chunk_channel.take());
        self.total_chunk_count = 0;
        self.chunks_received = 0;

        self.state = AppState::Setup;
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            //
            state: AppState::Setup,
            //
            selected_scene: "Book 1 final scene",
            scene_creators: scene_creators(),
            //
            image_width: 400,
            image_height: 400,
            sample_count: 10,
            max_depth: 10,
            chunk_size: 16,
            //
            tex_id: None,
            pixels: vec![],

            //
            total_chunk_count: 0,
            chunks_received: 0,
            chunk_channel: None,
            thread_pool: None,
        }
    }
}

impl epi::App for App {
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
    ) {
        let mut fonts = egui::FontDefinitions::default();

        for text_style in egui::TextStyle::all() {
            fonts
                .family_and_size
                .insert(text_style, (egui::FontFamily::Monospace, 16.0));
        }

        ctx.set_fonts(fonts);
    }

    fn update(
        &mut self,
        ctx: &egui::CtxRef,
        frame: &mut epi::Frame<'_>,
    ) {
        match self.state {
            AppState::Setup => self.update_when_setup(ctx, frame),
            AppState::Rendering => self.update_when_rendering(ctx, frame),
        }
        ctx.request_repaint();
    }

    fn name(&self) -> &str {
        "Raytracer"
    }
}

//
//
//
fn raytrace_task(
    sender: channel::Sender<Chunk>,
    scene: crate::raytracer::scene::Scene,
    image_width: usize,
    image_height: usize,
    sample_count: usize,
    max_depth: usize,
    chunk_size: usize,
) {
    //
    log::info!(
        "starting raytrace task: with params:\n    w = {}\n    h = {}\n    chunk = {}\n    samples = {}\n    max_depth = {}",
        image_width, image_height, chunk_size, sample_count, max_depth
    );

    //
    use rand::{thread_rng, Rng};

    use crate::{
        cgmath::*,
        raytracer::{
            camera::Camera,
            raytrace::{ray_color, RayCastOptions},
        },
    };

    use std::time::Instant;

    //
    let aspect_ratio = image_width as f32 / image_height as f32;

    let camera = {
        let eye = Vec3::new(13.0, 2.0, 3.0);
        let target = Vec3::ZERO;
        let up = Vec3::Y;
        let vertical_fov = Degrees(20.0).into();
        let aperture = 0.1;
        let focal_distance = 10.0;

        &Camera::new(
            eye,
            target,
            up,
            vertical_fov,
            aspect_ratio,
            aperture,
            focal_distance,
        )
    };

    //
    let scene = &scene;

    rayon::scope(move |s| {
        let x_chunks = (image_width + chunk_size - 1) / chunk_size;
        let y_chunks = (image_height + chunk_size - 1) / chunk_size;

        for y in 0..y_chunks {
            for x in 0..x_chunks {
                let sender = sender.clone();

                //
                s.spawn(move |_| {
                    let mut colors = vec![Color::from_rgb(0.0, 0.0, 0.0); chunk_size * chunk_size];

                    let begin = Instant::now();

                    for j in 0..chunk_size {
                        for i in 0..chunk_size {
                            let tx = x * chunk_size + i;
                            let ty = y * chunk_size + j;

                            if tx >= image_width || ty >= image_height {
                                continue;
                            }

                            let u = tx as f32 / (image_width - 1) as f32;
                            let v = 1.0 - (ty as f32 / (image_height - 1) as f32);

                            let mut comps = Vec3::ZERO;
                            for _ in 0..sample_count {
                                let du = thread_rng().gen_range(-0.5..0.5) / image_width as f32;
                                let dv = thread_rng().gen_range(-0.5..0.5) / image_height as f32;

                                let ray = camera.ray_at(u + du, v + dv);

                                let ray_cast_options = RayCastOptions {
                                    sample_count: sample_count,
                                    max_depth: max_depth,
                                };
                                comps = comps + ray_color(&ray_cast_options, scene, &ray, 0).into();
                            }

                            comps = comps / sample_count as f32;

                            // color correction ?
                            comps.x = comps.x.sqrt();
                            comps.y = comps.y.sqrt();
                            comps.z = comps.z.sqrt();

                            colors[j * chunk_size + i] = comps.into();
                        }
                    }

                    let end = Instant::now();

                    let _ = sender.send(Chunk {
                        x,
                        y,
                        duration: end - begin,
                        pixels: colors,
                    });
                });
            }
        }
    });
}
