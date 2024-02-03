use super::*;
#[cfg(feature = "egui")]
use egui::{Color32, ColorImage};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
#[serde(from = "VisualSerdeId", into = "VisualSerdeId")]
pub struct ImageVisual(VisualId);

impl_visual_serde_id_from_to_for_sub_visual_id! { ImageVisual }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryImageVisualData {
    pub bits_per_row: u8,
    pub width: u32,
    pub height: u32,
    /// the len must be a multiple of `bits_per_row`
    pub bitmap: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageVisualData {
    Binary(BinaryImageVisualData),
}

impl ImageVisual {
    pub fn new_binary_image(
        bits_per_row: u8,
        width: u32,
        height: u32,
        bitmap: Vec<u8>,
        sct: &mut VisualSynchrotron,
    ) -> Self {
        ImageVisual(
            sct.alloc_visual(ImageVisualData::Binary(BinaryImageVisualData {
                bits_per_row,
                width,
                height,
                bitmap,
            })),
        )
    }

    pub fn data<'a>(self, visual_synchrotron: &'a VisualSynchrotron) -> &'a ImageVisualData {
        let VisualData::Image(data) = self.0.data(visual_synchrotron) else {
            unreachable!()
        };
        data
    }

    #[cfg(feature = "egui")]
    pub fn color_image(self, visual_synchrotron: &VisualSynchrotron) -> ColorImage {
        self.data(visual_synchrotron).color_image()
    }
}

impl Visual {
    pub fn new_binary_image(
        bits_per_row: u8,
        width: u32,
        height: u32,
        bitmap: Vec<u8>,
        sct: &mut VisualSynchrotron,
    ) -> Self {
        ImageVisual::new_binary_image(bits_per_row, width, height, bitmap, sct).into()
    }
}

#[cfg(feature = "egui")]
impl ImageVisualData {
    pub fn color_image(&self) -> ColorImage {
        match self {
            ImageVisualData::Binary(slf) => slf.color_image(),
        }
    }
}

#[cfg(feature = "egui")]
impl BinaryImageVisualData {
    fn color_image(&self) -> ColorImage {
        let Self {
            width,
            height,
            bits_per_row,
            ref bitmap,
        } = *self;
        let pixels = (0..height)
            .into_iter()
            .map(move |i| {
                (0..width).into_iter().map(move |j| {
                    let j0 = j / 8;
                    let j1 = j % 8;
                    debug_assert_eq!(j, j0 * 8 + j1);
                    debug_assert!(j1 < 8);
                    let byte: u8 = bitmap[(i * (bits_per_row as u32) + j0) as usize];
                    let bit: bool = (byte & (1 << (7 - j1))) != 0;
                    match bit {
                        true => Color32::WHITE,
                        false => Color32::BLACK,
                    }
                })
            })
            .flatten()
            .collect();
        ColorImage {
            size: [width as usize, height as usize],
            pixels,
        }
    }
}

#[test]
fn image_visual_data_into_color_image_works() {
    use expect_test::{expect, expect_file};

    let ColorImage { size, pixels } = ImageVisualData::Binary(BinaryImageVisualData {
        bits_per_row: 2,
        width: 15,
        height: 4,
        bitmap: vec![1, 0, 11, 0, 31, 0, 51, 0],
    })
    .color_image();
    let size_expected = expect![[r#"
        [
            15,
            4,
        ]
    "#]];
    let pixels_expected = expect_file!["../../expect-files/image_visual_data_into_color_image.md"];
    size_expected.assert_debug_eq(&size);
    pixels_expected.assert_debug_eq(&pixels)
}
