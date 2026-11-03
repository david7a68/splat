//! `visual-compare <labelA> <labelB>`: which captures of two runs differ, by more than the
//! rendering noise. A simplify pass that changes no behaviour should report nothing.
//!
//! Software rendering is threaded, so a few pixels on a blended edge can differ by one level
//! between two runs of the same code. The tolerance is above that, and below anything a
//! person could see.
use image::RgbaImage;
use rayon::prelude::*;
use std::{fs, path::Path};

const DEFAULT_TOLERANCE: u8 = 2;

/// How many pixels differ by more than the tolerance in any channel, and the largest gap.
pub fn differing_pixels(a: &RgbaImage, b: &RgbaImage, tolerance: u8) -> (usize, u8) {
    // Image equality uses a bulk byte comparison. Most captures are identical;
    // retain the per-channel tolerance check for every image that is not.
    if a == b {
        return (0, 0);
    }
    let mut count = 0;
    let mut largest = 0;
    for (left, right) in a.pixels().zip(b.pixels()) {
        let gap = left
            .0
            .iter()
            .zip(&right.0)
            .map(|(l, r)| l.abs_diff(*r))
            .max()
            .unwrap_or(0);
        largest = largest.max(gap);
        if gap > tolerance {
            count += 1;
        }
    }
    (count, largest)
}

fn captures(directory: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("{}: {error}", directory.display()))
        .flatten()
        .filter_map(|file| file.file_name().into_string().ok())
        .filter(|name| name.ends_with(".png") && name.contains("__"))
        .collect();
    names.sort();
    names
}

/// The lines of the report: one for each file that differs.
pub fn compare(a: &Path, b: &Path, tolerance: u8) -> (usize, Vec<String>) {
    let (left, right) = (captures(a), captures(b));
    let mut lines = Vec::new();
    let mut compared = 0;
    let mut noise = 0;
    // Indexed collection keeps filename order while each worker owns only one pair.
    // Decode every pair, including equal PNGs; preserve validation and tolerance.
    let results: Vec<_> = left
        .par_iter()
        .map(|name| {
            if !right.contains(name) {
                return Ok((0, 0, Some(format!("only in A: {name}"))));
            }
            let open = |dir: &Path| {
                image::open(dir.join(name))
                    .map(|image| image.to_rgba8())
                    .map_err(|error| format!("{name}: {error}"))
            };
            let (one, other) = (open(a)?, open(b)?);
            if one.dimensions() != other.dimensions() {
                return Ok((1, 0, Some(format!("size differs: {name}"))));
            }
            let (count, largest) = differing_pixels(&one, &other, tolerance);
            let line =
                (count > 0).then(|| format!("{name}: {count} pixels differ, by up to {largest}"));
            Ok::<_, String>((1, usize::from(count == 0 && largest > 0), line))
        })
        .collect();
    for result in results {
        // Report the first decode error in the same sorted order as the serial loop.
        let (pair, within_tolerance, line) = result.unwrap_or_else(|error| panic!("{error}"));
        compared += pair;
        noise += within_tolerance;
        lines.extend(line);
    }
    for name in right.iter().filter(|name| !left.contains(name)) {
        lines.push(format!("only in B: {name}"));
    }
    println!("{compared} compared. {noise} differ only within the tolerance of {tolerance}.");
    (compared, lines)
}

/// Run by `devenv shell -- visual-compare <labelA> <labelB>`.
#[test]
#[ignore = "compares two earlier runs; the visual-compare script sets its folders"]
fn visual_compare() {
    let folder = |name: &str| {
        std::env::var_os(name).unwrap_or_else(|| panic!("{name} is not set. Use visual-compare."))
    };
    let tolerance = std::env::var("VISUAL_COMPARE_TOLERANCE")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(DEFAULT_TOLERANCE);
    let (a, b) = (folder("VISUAL_COMPARE_A"), folder("VISUAL_COMPARE_B"));
    let (_, lines) = compare(Path::new(&a), Path::new(&b), tolerance);
    for line in &lines {
        println!("{line}");
    }
    assert!(lines.is_empty(), "{} captures differ", lines.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;

    fn image_with(pixel: Rgba<u8>) -> RgbaImage {
        let mut image = RgbaImage::from_pixel(4, 4, Rgba([10, 10, 10, 255]));
        image.put_pixel(1, 1, pixel);
        image
    }

    #[test]
    fn noise_within_the_tolerance_is_not_a_difference() {
        let base = image_with(Rgba([10, 10, 10, 255]));
        assert_eq!(
            differing_pixels(&base, &image_with(Rgba([11, 12, 10, 255])), 2),
            (0, 2)
        );
    }

    #[test]
    fn a_change_above_the_tolerance_is_reported() {
        let base = image_with(Rgba([10, 10, 10, 255]));
        assert_eq!(
            differing_pixels(&base, &image_with(Rgba([13, 10, 10, 255])), 2),
            (1, 3)
        );
    }

    #[test]
    fn parallel_comparison_preserves_counts_and_report_order() {
        let a = tempfile::tempdir().unwrap();
        let b = tempfile::tempdir().unwrap();
        let base = image_with(Rgba([10, 10, 10, 255]));
        for name in [
            "a__equal.png",
            "b__noise.png",
            "c__change.png",
            "d__size.png",
        ] {
            base.save(a.path().join(name)).unwrap();
            base.save(b.path().join(name)).unwrap();
        }
        image_with(Rgba([11, 12, 10, 255]))
            .save(b.path().join("b__noise.png"))
            .unwrap();
        image_with(Rgba([13, 10, 10, 255]))
            .save(b.path().join("c__change.png"))
            .unwrap();
        RgbaImage::new(2, 2)
            .save(b.path().join("d__size.png"))
            .unwrap();
        base.save(a.path().join("e__left.png")).unwrap();
        base.save(b.path().join("f__right.png")).unwrap();
        assert_eq!(
            compare(a.path(), b.path(), 2),
            (
                4,
                vec![
                    "c__change.png: 1 pixels differ, by up to 3".to_owned(),
                    "size differs: d__size.png".to_owned(),
                    "only in A: e__left.png".to_owned(),
                    "only in B: f__right.png".to_owned(),
                ]
            )
        );
    }

    #[test]
    #[should_panic(expected = "a__invalid.png:")]
    fn equal_invalid_pngs_still_fail_decoding() {
        let a = tempfile::tempdir().unwrap();
        let b = tempfile::tempdir().unwrap();
        for directory in [a.path(), b.path()] {
            fs::write(directory.join("a__invalid.png"), b"not a PNG").unwrap();
            fs::write(directory.join("b__invalid.png"), b"not a PNG").unwrap();
        }
        compare(a.path(), b.path(), 2);
    }
}
