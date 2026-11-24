//! Finds art: images from registered packages, and cached character portraits.
//!
//! The index is kept in the state context, so the one Reader and the Markdown view can find
//! art wherever they are shown, without each page passing it down. The GPUI views load and
//! cache the images themselves (`gpui::img`).
use crate::persistence::Module;
use crate::state_context::{Slot, StateContext};
use splat::content::{Entry, art::ArtIndex};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, mpsc},
};

/// Keeps the art index in step with the registered packages.
#[derive(Default)]
pub struct Loader {
    roots: Vec<PathBuf>,
    pending: Option<mpsc::Receiver<ArtIndex>>,
}

impl Loader {
    #[cfg(any(test, feature = "test-support"))]
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }

    pub fn poll(&mut self, ctx: &StateContext, modules: &[Module]) {
        if !modules.iter().map(|m| &m.path).eq(self.roots.iter()) {
            self.roots = modules.iter().map(|m| m.path.clone()).collect();
            let roots = self.roots.clone();
            let (tx, rx) = mpsc::channel();
            self.pending = Some(rx);
            let waker = ctx.waker();
            std::thread::spawn(move || {
                let _ = tx.send(ArtIndex::load(roots.iter().map(PathBuf::as_path)));
                waker.request_repaint();
            });
        }
        if let Some(index) = self.pending.as_ref().and_then(|rx| rx.try_recv().ok()) {
            self.pending = None;
            ctx.set(INDEX, Arc::new(index));
        }
    }
}

/// The art index in use.
const INDEX: Slot<Arc<ArtIndex>> = Slot::new("art_index");

fn index(ctx: &StateContext) -> Option<Arc<ArtIndex>> {
    ctx.get(INDEX)
}

/// The art for an entry: its own, the art of what it was copied from, or for a creature, the
/// default picture for its type.
pub fn entry_art(ctx: &StateContext, entry: &Entry) -> Option<PathBuf> {
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

/// The generation of the art index in use (`ArtIndex::generation`), for a reader that keeps its
/// figures (layout PR 9b-R1a); `None` before an index is loaded.
pub fn generation(ctx: &StateContext) -> Option<u64> {
    index(ctx).map(|index| index.generation())
}

/// The local copy of the image at a link in a note.
pub fn link_art(ctx: &StateContext, url: &str) -> Option<PathBuf> {
    index(ctx)?.link(url).map(Path::to_path_buf)
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn the_loader_follows_the_registered_packages() {
        let (root, _) = package();
        let ctx = StateContext::new();
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
        let ctx = StateContext::new();
        let index = Arc::new(ArtIndex::load([root.path()]));
        ctx.set(INDEX, index);
        let creature = |fields: &str| {
            let text = splat::content::formats::fixture(
                "Fixture.md",
                &format!(
                    "splat_id: {}\nsplat_kind: creature\nsplat_name: C\n{fields}",
                    uuid::Uuid::new_v4()
                ),
                "",
            )
            .expect("schema-2 fixture");
            Entry::parse("c.md", &text).unwrap()
        };
        let art = |fields: &str| {
            let path = entry_art(&ctx, &creature(fields))?;
            Some(path.file_name()?.to_str()?.to_owned())
        };
        let beast = "splat_creature: {size: medium, type: beast}\n";
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
}
