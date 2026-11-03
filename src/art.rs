//! Draws art: images from registered packages, and cached character portraits.
//!
//! The index is kept in the egui context, so the one Reader and the Markdown view can find
//! art wherever they are shown, without each page passing it down.
use crate::{
    design::{self, Type, colors},
    persistence::Module,
};
use eframe::egui::{self, Id, Rect, Vec2, vec2};
use splat::content::{Entry, art::ArtIndex};
use std::{
    collections::{HashMap, VecDeque},
    path::{Path, PathBuf},
    sync::{Arc, mpsc},
};

/// Decoded images kept ready. Older ones are dropped, so browsing does not fill video memory.
const KEEP_DECODED: usize = 32;
const HERO: Vec2 = vec2(360.0, 320.0);
/// Beside a run or in a narrow page, the rules come first and the picture stays small.
const SMALL_HERO_HEIGHT: f32 = 110.0;
const FIGURE_HEIGHT: f32 = 520.0;

pub fn install(ctx: &egui::Context) {
    egui_extras::install_image_loaders(ctx);
}

/// Keeps the art index in step with the registered packages.
#[derive(Default)]
pub struct Loader {
    roots: Vec<PathBuf>,
    pending: Option<mpsc::Receiver<ArtIndex>>,
}

impl Loader {
    pub fn poll(&mut self, ctx: &egui::Context, modules: &[Module]) {
        if !modules.iter().map(|m| &m.path).eq(self.roots.iter()) {
            self.roots = modules.iter().map(|m| m.path.clone()).collect();
            let roots = self.roots.clone();
            let (tx, rx) = mpsc::channel();
            self.pending = Some(rx);
            let ctx = ctx.clone();
            std::thread::spawn(move || {
                let _ = tx.send(ArtIndex::load(roots.iter().map(PathBuf::as_path)));
                ctx.request_repaint();
            });
        }
        if let Some(index) = self.pending.as_ref().and_then(|rx| rx.try_recv().ok()) {
            self.pending = None;
            ctx.data_mut(|data| data.insert_temp(index_id(), Arc::new(index)));
        }
    }
}

fn index_id() -> Id {
    Id::new("art_index")
}

fn index(ctx: &egui::Context) -> Option<Arc<ArtIndex>> {
    ctx.data(|data| data.get_temp(index_id()))
}

/// The art for an entry: its own, the art of what it was copied from, or for a creature, the
/// default picture for its type.
pub fn entry_art(ctx: &egui::Context, entry: &Entry) -> Option<PathBuf> {
    let index = index(ctx)?;
    let origin = entry
        .fields
        .get("splat_origin")
        .and_then(|origin| origin.get("key")?.as_str());
    let creature_type = entry
        .fields
        .get("splat_creature")
        .and_then(|creature| creature.get("type")?.as_str());
    entry
        .key
        .as_deref()
        .and_then(|key| index.entry(key))
        .or_else(|| index.entry(origin?))
        .or_else(|| index.creature_type(creature_type?))
        .map(Path::to_path_buf)
}

/// The local copy of the image at a link in a note.
pub fn link_art(ctx: &egui::Context, url: &str) -> Option<PathBuf> {
    index(ctx)?.link(url).map(Path::to_path_buf)
}

fn uri(path: &Path) -> String {
    format!("file://{}", path.display())
}

/// Note that an image is in use, and drop the oldest if too many are decoded.
fn remember(ctx: &egui::Context, uri: &str) {
    let dropped = ctx.data_mut(|data| {
        let recent =
            data.get_temp_mut_or_default::<VecDeque<String>>(Id::new("art_recently_shown"));
        recent.retain(|known| known != uri);
        recent.push_back(uri.to_owned());
        (recent.len() > KEEP_DECODED)
            .then(|| recent.pop_front())
            .flatten()
    });
    if let Some(uri) = dropped {
        ctx.forget_image(&uri);
        // egui treats each WebP as an animation and keeps its texture under the frame's address.
        ctx.forget_image(&format!("{uri}#0"));
    }
}

/// An image's size, from its file header. Knowing it lets the page keep its layout while
/// the image loads, and leave images out of view undecoded.
fn dimensions(ctx: &egui::Context, path: &Path) -> Option<Vec2> {
    let id = Id::new("art_dimensions");
    if let Some(known) = ctx.data_mut(|data| {
        data.get_temp_mut_or_default::<HashMap<PathBuf, Option<Vec2>>>(id)
            .get(path)
            .copied()
    }) {
        return known;
    }
    let size = image::image_dimensions(path)
        .ok()
        .map(|(width, height)| vec2(width as f32, height as f32));
    ctx.data_mut(|data| {
        data.get_temp_mut_or_default::<HashMap<PathBuf, Option<Vec2>>>(id)
            .insert(path.to_path_buf(), size);
    });
    size
}

/// The size to draw an image at. A small image is not enlarged.
fn fitted(size: Vec2, max: Vec2) -> Vec2 {
    size * (max.x / size.x).min(max.y / size.y).min(1.0)
}

fn picture(ui: &mut egui::Ui, path: &Path, name: &str, max: Vec2) {
    let Some(size) = dimensions(ui.ctx(), path).filter(|size| size.min_elem() > 0.0) else {
        return;
    };
    let (rect, response) = ui.allocate_exact_size(fitted(size, max), egui::Sense::click());
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), name));
    if ui.is_rect_visible(rect) {
        let uri = uri(path);
        remember(ui.ctx(), &uri);
        egui::Image::from_uri(uri)
            .corner_radius(design::RADIUS_CONTROL)
            .paint_at(ui, rect);
        if response.has_focus() {
            design::focus_ring(ui, rect);
        }
    }
    let response = response
        .on_hover_text("View full size")
        .on_hover_cursor(egui::CursorIcon::ZoomIn);
    if response.clicked() {
        ui.ctx().data_mut(|data| {
            data.insert_temp(viewing_id(), Some((path.to_path_buf(), name.to_owned())));
        });
    }
}

/// An entry's picture, in the Reader's margin.
pub fn hero(ui: &mut egui::Ui, path: &Path, name: &str) {
    let width = ui.available_width().min(HERO.x);
    picture(ui, path, name, vec2(width, HERO.y));
    ui.add_space(design::S);
}

/// An entry's picture, above its text where there is no margin.
pub fn small_hero(ui: &mut egui::Ui, path: &Path, name: &str) {
    ui.add_space(design::S);
    let width = ui.available_width();
    picture(ui, path, name, vec2(width, SMALL_HERO_HEIGHT));
    ui.add_space(design::XS);
}

/// An image in the flow of a note, as wide as the text.
pub fn figure(ui: &mut egui::Ui, path: &Path, caption: &str) {
    let name = match caption.trim() {
        "" => "Image",
        caption => caption,
    };
    picture(ui, path, name, vec2(ui.available_width(), FIGURE_HEIGHT));
    ui.add_space(design::XS);
}

fn viewing_id() -> Id {
    Id::new("art_viewing")
}

fn viewing(ctx: &egui::Context) -> Option<(PathBuf, String)> {
    ctx.data(|data| data.get_temp(viewing_id())).flatten()
}

/// True while an image fills the window. Global shortcuts must leave Escape to it.
pub fn viewer_open(ctx: &egui::Context) -> bool {
    viewing(ctx).is_some()
}

/// The full-window view of the image the DM clicked, if any.
pub fn show_viewer(ctx: &egui::Context) {
    if let Some((path, name)) = viewing(ctx) {
        let uri = uri(&path);
        remember(ctx, &uri);
        let viewport = ctx.content_rect();
        let max = viewport.size()
            - vec2(
                design::XXL * 2.0,
                design::XXL * 2.0
                    + design::Density::Scan.control()
                    + Type::Small.line()
                    + design::S,
            );
        let Some(size) = dimensions(ctx, &path) else {
            if design::image_viewer(ctx, &name, &uri) {
                ctx.data_mut(|data| data.remove::<Option<(PathBuf, String)>>(viewing_id()));
            }
            return;
        };
        let size = fitted(size, max);
        let response = egui::Modal::new(Id::new("image_viewer"))
            .backdrop_color(colors().rail)
            .frame(egui::Frame::new())
            .show(ctx, |ui| {
                ui.set_width(size.x);
                let mut close = false;
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        close = design::Action::quiet("Close")
                            .icon(design::icons::CLOSE)
                            .key("Esc")
                            .show(ui)
                            .clicked();
                    });
                });
                let image = ui
                    .add(
                        egui::Image::from_uri(&uri)
                            .fit_to_exact_size(size)
                            .sense(egui::Sense::click()),
                    )
                    .on_hover_text(&name);
                image.widget_info(|| {
                    egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &name)
                });
                ui.label(Type::Small.rich(ui, &name).color(colors().text));
                close || image.clicked()
            });
        let closed = response.should_close() || response.inner;
        if closed {
            ctx.data_mut(|data| data.remove::<Option<(PathBuf, String)>>(viewing_id()));
        }
    }
}

/// The square of an image to show in a round picture. A tall image keeps its top, where a
/// creature's head usually is.
fn square_crop(size: Vec2) -> Rect {
    let side = size.min_elem();
    let width = side / size.x;
    let height = side / size.y;
    Rect::from_min_size(egui::pos2((1.0 - width) / 2.0, 0.0), vec2(width, height))
}

/// A round picture of a character or creature, or the first letter of the name if there is
/// no picture. `dim` darkens it, for someone who is not present.
pub fn portrait(
    ui: &egui::Ui,
    rect: Rect,
    path: Option<&Path>,
    name: &str,
    dim: Option<egui::Color32>,
) {
    let c = colors();
    if let Some(path) = path {
        let uri = uri(path);
        remember(ui.ctx(), &uri);
        let mut image = egui::Image::from_uri(uri).corner_radius(rect.width() / 2.0);
        if let Some(size) = dimensions(ui.ctx(), path).filter(|size| size.min_elem() > 0.0) {
            image = image.uv(square_crop(size));
        }
        if let Some(dim) = dim {
            image = image.tint(dim);
        }
        image.paint_at(ui, rect);
    } else {
        ui.painter()
            .circle_filled(rect.center(), rect.width() / 2.0, c.hover);
        let initial = name.chars().next().map(char::to_uppercase);
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            initial.map_or_else(String::new, |letters| letters.collect()),
            Type::Name.font(ui),
            c.muted,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::{Harness, kittest::Queryable};
    use std::fs;

    /// A package with one image for a link, and the art index loaded from it.
    fn package() -> (tempfile::TempDir, Arc<ArtIndex>) {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir(root.path().join("Figures")).unwrap();
        image::DynamicImage::new_rgb8(40, 20)
            .save(root.path().join("Figures/map.png"))
            .unwrap();
        fs::write(
            root.path().join(splat::content::art::ART_FILE),
            "```json\n{\"splat_art\":1,\"links\":{\"https://a/map.png\":\"Figures/map.png\"}}\n```\n",
        )
        .unwrap();
        let index = Arc::new(ArtIndex::load([root.path()]));
        (root, index)
    }

    #[test]
    fn a_note_shows_a_linked_image_with_a_local_copy_and_it_opens_full_size() {
        let (_root, index) = package();
        let mut harness = Harness::builder().build_ui(|ui| {
            crate::markdown_view::show(ui, "*The keep* ([image](https://a/map.png))");
            show_viewer(ui.ctx());
        });
        install(&harness.ctx);
        harness
            .ctx
            .data_mut(|data| data.insert_temp(index_id(), index));
        harness.run_ok();
        harness.get_by_label("The keep").click();
        harness.step();
        assert_eq!(viewing(&harness.ctx).unwrap().1, "The keep");
        harness.step();
        harness.key_press(egui::Key::Escape);
        harness.step();
        harness.step();
        assert!(!viewer_open(&harness.ctx));
    }

    #[test]
    fn the_loader_follows_the_registered_packages() {
        let (root, _) = package();
        let ctx = egui::Context::default();
        let mut loader = Loader::default();
        let module = Module {
            id: uuid::Uuid::new_v4(),
            name: "Art".into(),
            path: root.path().to_path_buf(),
        };
        for _ in 0..500 {
            loader.poll(&ctx, std::slice::from_ref(&module));
            if link_art(&ctx, "https://a/map.png").is_some() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert!(link_art(&ctx, "https://a/map.png").is_some());
        loader.poll(&ctx, &[]);
        for _ in 0..500 {
            loader.poll(&ctx, &[]);
            if link_art(&ctx, "https://a/map.png").is_none() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert!(link_art(&ctx, "https://a/map.png").is_none());
    }

    #[test]
    fn a_link_with_no_local_copy_stays_a_link() {
        let (_root, index) = package();
        let mut harness = Harness::builder().build_ui(|ui| {
            crate::markdown_view::show(ui, "*The moat* ([image](https://a/moat.png))");
        });
        harness
            .ctx
            .data_mut(|data| data.insert_temp(index_id(), index));
        harness.run_ok();
        assert!(harness.query_by_label("The moat").is_none());
        assert!(harness.query_by_label("The moat (image)").is_some());
    }

    #[test]
    fn the_image_used_longest_ago_is_dropped_first() {
        let ctx = egui::Context::default();
        for number in 0..KEEP_DECODED {
            remember(&ctx, &number.to_string());
        }
        remember(&ctx, "0");
        remember(&ctx, "new");
        let recent = ctx.data_mut(|data| {
            data.get_temp_mut_or_default::<VecDeque<String>>(Id::new("art_recently_shown"))
                .clone()
        });
        assert_eq!(recent.len(), KEEP_DECODED);
        assert!(recent.contains(&"0".to_owned()));
        assert!(!recent.contains(&"1".to_owned()));
    }

    /// Draw real art to a file, to look at. Set SPLAT_ART_PACKAGE to the package folder.
    #[test]
    #[ignore = "needs an art package on this machine"]
    fn preview() {
        use splat::content::Entry;
        let root = PathBuf::from(std::env::var("SPLAT_ART_PACKAGE").unwrap());
        let index = Arc::new(ArtIndex::load([root.as_path()]));
        assert!(!index.is_empty());
        let shown = index.clone();
        let goblin =
            splat::content::definitions::source_file("definitions/creatures/goblin_warrior.md")
                .unwrap();
        let goblin = Entry::parse("goblin.md", goblin).unwrap();
        let figure = std::env::var("SPLAT_ART_LINK").unwrap();
        let note =
            format!("*A figure from a chapter* ([image]({figure}))\n\nText after the figure.");
        let portrait_path = std::env::var("SPLAT_ART_PORTRAIT").ok().map(PathBuf::from);
        let mut harness = Harness::builder()
            .with_size(vec2(1300.0, 1000.0))
            .build_ui(|ui| {
                ui.columns(2, |columns| {
                    let origin = crate::library::view::Origin {
                        path: "x",
                        owner: "x",
                        builtin: true,
                    };
                    crate::library::view::show(
                        &mut columns[0],
                        &goblin.name,
                        Some(&goblin),
                        &goblin.body,
                        &origin,
                        &crate::library::view::NoLinks,
                    );
                    crate::markdown_view::show(&mut columns[1], &note);
                    let (rect, _) =
                        columns[1].allocate_exact_size(vec2(56.0, 56.0), egui::Sense::empty());
                    portrait(&columns[1], rect, portrait_path.as_deref(), "Tiny", None);
                    let (rect, _) =
                        columns[1].allocate_exact_size(vec2(38.0, 38.0), egui::Sense::empty());
                    portrait(&columns[1], rect, None, "Morana", None);
                    columns[1].horizontal_wrapped(|ui| {
                        let words = std::env::var("SPLAT_ART_TYPES").unwrap_or_default();
                        for word in words.split(',') {
                            for side in [28.0, 56.0] {
                                let (rect, _) =
                                    ui.allocate_exact_size(vec2(side, side), egui::Sense::empty());
                                portrait(ui, rect, shown.creature_type(word), word, None);
                            }
                        }
                    });
                });
            });
        install(&harness.ctx);
        harness
            .ctx
            .data_mut(|data| data.insert_temp(index_id(), index));
        for _ in 0..200 {
            harness.step();
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        harness
            .render()
            .unwrap()
            .save(std::env::var("SPLAT_ART_OUT").unwrap())
            .unwrap();
    }

    #[test]
    fn a_round_picture_shows_the_top_of_a_tall_image_and_the_middle_of_a_wide_one() {
        let tall = square_crop(vec2(100.0, 400.0));
        assert_eq!(tall.min, egui::pos2(0.0, 0.0));
        assert_eq!(tall.max, egui::pos2(1.0, 0.25));
        let wide = square_crop(vec2(400.0, 100.0));
        assert_eq!(wide.min, egui::pos2(0.375, 0.0));
        assert_eq!(wide.max, egui::pos2(0.625, 1.0));
    }

    #[test]
    fn a_creature_falls_back_to_its_origin_and_then_to_its_type() {
        let root = tempfile::tempdir().unwrap();
        for name in ["own.png", "origin.png", "beast.png"] {
            image::DynamicImage::new_rgb8(4, 4)
                .save(root.path().join(name))
                .unwrap();
        }
        fs::write(
            root.path().join(splat::content::art::ART_FILE),
            "```json\n{\"splat_art\":1,\"entries\":{\"pack:creatures/own\":\"own.png\",\"pack:creatures/origin\":\"origin.png\"},\"creature_types\":{\"beast\":\"beast.png\"}}\n```\n",
        )
        .unwrap();
        let ctx = egui::Context::default();
        let index = Arc::new(ArtIndex::load([root.path()]));
        ctx.data_mut(|data| data.insert_temp(index_id(), index));
        let creature = |fields: &str| {
            let text = format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: C\n{fields}---\n",
                uuid::Uuid::new_v4()
            );
            Entry::parse("c.md", &text).unwrap()
        };
        let art = |fields: &str| {
            let path = entry_art(&ctx, &creature(fields))?;
            Some(path.file_name()?.to_str()?.to_owned())
        };
        let beast = "splat_creature: {size: medium, type: Beast}\n";
        assert_eq!(
            art(&format!("splat_key: pack:creatures/own\n{beast}")).as_deref(),
            Some("own.png")
        );
        assert_eq!(
            art(&format!(
                "splat_key: pack:creatures/copy\nsplat_origin: {{key: pack:creatures/origin}}\n{beast}"
            ))
            .as_deref(),
            Some("origin.png")
        );
        assert_eq!(art(beast).as_deref(), Some("beast.png"));
        assert_eq!(art("splat_creature: {size: medium, type: ooze}\n"), None);
    }

    #[test]
    fn a_large_image_shrinks_to_fit_and_a_small_one_keeps_its_size() {
        assert_eq!(
            fitted(vec2(1000.0, 500.0), vec2(500.0, 500.0)),
            vec2(500.0, 250.0)
        );
        assert_eq!(
            fitted(vec2(500.0, 1000.0), vec2(500.0, 250.0)),
            vec2(125.0, 250.0)
        );
        assert_eq!(
            fitted(vec2(100.0, 50.0), vec2(500.0, 500.0)),
            vec2(100.0, 50.0)
        );
    }
}
