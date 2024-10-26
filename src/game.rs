use std::fmt::Display;

use num_traits::{Num, NumCast};
use palette::{color_difference::ImprovedCiede2000, Hsl, Hsv, IntoColor, Lab, Lch, Srgb, Xyz};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Slider<T: Num + Into<f32>> {
    pub name: &'static str,
    pub value: T,
    pub min: T,
    pub max: T,
}

impl<T: Num + Into<f32> + NumCast + Copy> Slider<T> {
    pub fn new_linear(name: &'static str, value: T, min: T, max: T) -> Self {
        Slider {
            name,
            value,
            min,
            max,
        }
    }
}

pub struct ColorSpaceMeta {
    pub name: &'static str,
    pub info_link: &'static str,
    pub slider_names: &'static [&'static str],
}

pub struct ColorChallenge {
    target: Srgb,
}

#[derive(Debug, Clone, Copy)]
pub enum ExcludeReason {
    TooDark,
    TooBright,
    LowSaturation,
    HighSaturation,
}

impl Display for ExcludeReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExcludeReason::TooDark => write!(f, "Too dark"),
            ExcludeReason::TooBright => write!(f, "Too bright"),
            ExcludeReason::LowSaturation => write!(f, "Low saturation"),
            ExcludeReason::HighSaturation => write!(f, "High saturation"),
        }
    }
}

impl ColorChallenge {
    pub fn is_excluded(color: &Srgb) -> Option<ExcludeReason> {
        if color.blue + color.green + color.red < 0.08 * 3. {
            return Some(ExcludeReason::TooDark);
        }
        if color.blue + color.green + color.red > 0.92 * 3. {
            return Some(ExcludeReason::TooBright);
        }
        let hsv: Hsv = (*color).into_color();
        if hsv.saturation < 0.15 {
            return Some(ExcludeReason::LowSaturation);
        } else if hsv.saturation > 0.9 {
            return Some(ExcludeReason::HighSaturation);
        }
        None
    }

    pub fn target_color(&self) -> &Srgb {
        &self.target
    }

    pub fn new<R: Rng>(rng: &mut R) -> Self {
        let target = Srgb::new(rng.gen(), rng.gen(), rng.gen());

        // avoid colors that are too dark or too bright
        if Self::is_excluded(&target).is_some() {
            return Self::new(rng);
        }

        ColorChallenge { target }
    }

    pub fn compute_distance(&self, input: impl IntoColor<Lab>) -> f32 {
        let target_lab: Lab = self.target.into_color();
        let input_lab: Lab = input.into_color();

        return target_lab.improved_difference(input_lab);
    }
}

pub fn chooser_convert<S: ColorChooser<f32>, D: ColorChooser<f32>>(
    source: &S,
    _dst: &D,
    sliders: &[Slider<f32>],
    dst_sliders: &mut [Slider<f32>],
) {
    source.convert_to::<D>(sliders, dst_sliders);
}

pub trait ColorChooser<T: Num + Copy + Into<f32>>: Default {
    fn get_meta(&self) -> ColorSpaceMeta;

    fn init_sliders(&self) -> Box<[Slider<T>]>;

    fn as_srgb(&self, sliders: &[Slider<T>]) -> Srgb;

    fn compute_xyz(&self, sliders: &[Slider<T>]) -> Xyz;
    fn compute_lab(&self, sliders: &[Slider<T>]) -> Lab;

    fn from_srgb(srgb: Srgb) -> Box<[Slider<T>]>;

    fn convert_to<C: ColorChooser<T>>(&self, sliders: &[Slider<T>], dst: &mut [Slider<T>]) {
        let srgb = self.as_srgb(sliders);
        dst.clone_from_slice(&C::from_srgb(srgb));
    }
}

#[derive(Default, Clone, Copy)]
pub struct RGBChooser;

impl ColorChooser<f32> for RGBChooser {
    fn get_meta(&self) -> ColorSpaceMeta {
        ColorSpaceMeta {
            name: "RGB",
            info_link: "https://en.wikipedia.org/wiki/RGB_color_model",
            slider_names: &["Red", "Green", "Blue"],
        }
    }

    fn init_sliders(&self) -> Box<[Slider<f32>]> {
        vec![
            Slider::new_linear("R", 0.5, 0.0, 1.0),
            Slider::new_linear("G", 0.5, 0.0, 1.0),
            Slider::new_linear("B", 0.5, 0.0, 1.0),
        ]
        .into_boxed_slice()
    }

    fn compute_xyz(&self, sliders: &[Slider<f32>]) -> Xyz {
        let srgb: Srgb = self.as_srgb(sliders).into_color();
        srgb.into_color()
    }

    fn compute_lab(&self, sliders: &[Slider<f32>]) -> Lab {
        let srgb: Srgb = self.as_srgb(sliders).into_color();
        srgb.into_color()
    }

    fn from_srgb(srgb: Srgb) -> Box<[Slider<f32>]> {
        vec![
            Slider::new_linear("R", srgb.red, 0.0, 1.0),
            Slider::new_linear("G", srgb.green, 0.0, 1.0),
            Slider::new_linear("B", srgb.blue, 0.0, 1.0),
        ]
        .into_boxed_slice()
    }

    fn as_srgb(&self, sliders: &[Slider<f32>]) -> Srgb {
        Srgb::new(sliders[0].value, sliders[1].value, sliders[2].value)
    }
}

#[derive(Default, Clone, Copy)]
pub struct HSVChooser;

impl ColorChooser<f32> for HSVChooser {
    fn get_meta(&self) -> ColorSpaceMeta {
        ColorSpaceMeta {
            name: "HSV",
            info_link: "https://en.wikipedia.org/wiki/HSL_and_HSV",
            slider_names: &["Hue", "Saturation", "Value"],
        }
    }

    fn init_sliders(&self) -> Box<[Slider<f32>]> {
        vec![
            Slider::new_linear("H", 180., 0.0, 360.0),
            Slider::new_linear("S", 0.5, 0.0, 1.0),
            Slider::new_linear("V", 0.5, 0.0, 1.0),
        ]
        .into_boxed_slice()
    }

    fn as_srgb(&self, sliders: &[Slider<f32>]) -> Srgb {
        let hsv = Hsv::new(sliders[0].value, sliders[1].value, sliders[2].value);
        hsv.into_color()
    }

    fn compute_xyz(&self, sliders: &[Slider<f32>]) -> Xyz {
        let srgb: Srgb = self.as_srgb(sliders).into_color();
        srgb.into_color()
    }

    fn compute_lab(&self, sliders: &[Slider<f32>]) -> Lab {
        let srgb: Srgb = self.as_srgb(sliders).into_color();
        srgb.into_color()
    }

    fn from_srgb(srgb: Srgb) -> Box<[Slider<f32>]> {
        let hsv: Hsv = srgb.into_color();
        vec![
            Slider::new_linear("H", hsv.hue.into_positive_degrees(), 0.0, 360.0),
            Slider::new_linear("S", hsv.saturation, 0.0, 1.0),
            Slider::new_linear("V", hsv.value, 0.0, 1.0),
        ]
        .into_boxed_slice()
    }
}

#[derive(Default, Clone, Copy)]
pub struct HSLChooser;

impl ColorChooser<f32> for HSLChooser {
    fn get_meta(&self) -> ColorSpaceMeta {
        ColorSpaceMeta {
            name: "HSL",
            info_link: "https://en.wikipedia.org/wiki/HSL_and_HSV",
            slider_names: &["Hue", "Saturation", "Lightness"],
        }
    }

    fn init_sliders(&self) -> Box<[Slider<f32>]> {
        vec![
            Slider::new_linear("H", 180., 0.0, 360.0),
            Slider::new_linear("S", 0.5, 0.0, 1.0),
            Slider::new_linear("L", 0.5, 0.0, 1.0),
        ]
        .into_boxed_slice()
    }

    fn as_srgb(&self, sliders: &[Slider<f32>]) -> Srgb {
        let hsv = Hsv::new(sliders[0].value, sliders[1].value, sliders[2].value);
        hsv.into_color()
    }

    fn compute_xyz(&self, sliders: &[Slider<f32>]) -> Xyz {
        let srgb: Srgb = self.as_srgb(sliders).into_color();
        srgb.into_color()
    }

    fn compute_lab(&self, sliders: &[Slider<f32>]) -> Lab {
        let srgb: Srgb = self.as_srgb(sliders).into_color();
        srgb.into_color()
    }

    fn from_srgb(srgb: Srgb) -> Box<[Slider<f32>]> {
        let hsl: Hsl = srgb.into_color();

        vec![
            Slider::new_linear("H", hsl.hue.into_positive_degrees(), 0.0, 360.0),
            Slider::new_linear("S", hsl.saturation, 0.0, 1.0),
            Slider::new_linear("L", hsl.lightness, 0.0, 1.0),
        ]
        .into_boxed_slice()
    }
}

#[derive(Default, Clone, Copy)]
pub struct LABChooser;

impl ColorChooser<f32> for LABChooser {
    fn get_meta(&self) -> ColorSpaceMeta {
        ColorSpaceMeta {
            name: "CIELAB",
            info_link: "https://en.wikipedia.org/wiki/CIELAB_color_space",
            slider_names: &["Lightness", "A", "B"],
        }
    }

    fn init_sliders(&self) -> Box<[Slider<f32>]> {
        vec![
            Slider::new_linear("L", 50., 0.0, 100.0),
            Slider::new_linear("a", 64., -128.0, 128.0),
            Slider::new_linear("b", 64., -128.0, 128.0),
        ]
        .into_boxed_slice()
    }

    fn as_srgb(&self, sliders: &[Slider<f32>]) -> Srgb {
        let lab = Lab::new(sliders[0].value, sliders[1].value, sliders[2].value);
        lab.into_color()
    }

    fn compute_xyz(&self, sliders: &[Slider<f32>]) -> Xyz {
        let lab = Lab::new(sliders[0].value, sliders[1].value, sliders[2].value);
        lab.into_color()
    }

    fn compute_lab(&self, sliders: &[Slider<f32>]) -> Lab {
        let lab = Lab::new(sliders[0].value, sliders[1].value, sliders[2].value);
        lab.into_color()
    }

    fn from_srgb(srgb: Srgb) -> Box<[Slider<f32>]> {
        let lab: Lab = srgb.into_color();
        vec![
            Slider::new_linear("L", lab.l, 0.0, 100.0),
            Slider::new_linear("a", lab.a, -128.0, 128.0),
            Slider::new_linear("b", lab.b, -128.0, 128.0),
        ]
        .into_boxed_slice()
    }
}

#[derive(Default, Clone, Copy)]
pub struct XYZChooser;

impl ColorChooser<f32> for XYZChooser {
    fn get_meta(&self) -> ColorSpaceMeta {
        ColorSpaceMeta {
            name: "CIEXYZ",
            info_link: "https://en.wikipedia.org/wiki/CIE_1931_color_space",
            slider_names: &["X", "Y", "Z"],
        }
    }

    fn init_sliders(&self) -> Box<[Slider<f32>]> {
        vec![
            Slider::new_linear("x", 0.5, 0.0, 1.0),
            Slider::new_linear("y", 0.5, 0.0, 1.0),
            Slider::new_linear("z", 0.5, 0.0, 1.0),
        ]
        .into_boxed_slice()
    }

    fn as_srgb(&self, sliders: &[Slider<f32>]) -> Srgb {
        let xyz = Xyz::new(sliders[0].value, sliders[1].value, sliders[2].value);
        xyz.into_color()
    }

    fn compute_xyz(&self, sliders: &[Slider<f32>]) -> Xyz {
        let xyz = Xyz::new(sliders[0].value, sliders[1].value, sliders[2].value);
        xyz.into_color()
    }

    fn compute_lab(&self, sliders: &[Slider<f32>]) -> Lab {
        let xyz = Xyz::new(sliders[0].value, sliders[1].value, sliders[2].value);
        xyz.into_color()
    }

    fn from_srgb(srgb: Srgb) -> Box<[Slider<f32>]> {
        let xyz: Xyz = srgb.into_color();
        vec![
            Slider::new_linear("x", xyz.x, 0.0, 1.0),
            Slider::new_linear("y", xyz.y, 0.0, 1.0),
            Slider::new_linear("z", xyz.z, 0.0, 1.0),
        ]
        .into_boxed_slice()
    }
}

#[derive(Default, Clone, Copy)]
pub struct LCHChooser;

impl ColorChooser<f32> for LCHChooser {
    fn get_meta(&self) -> ColorSpaceMeta {
        ColorSpaceMeta {
            name: "CIELCH",
            info_link: "https://en.wikipedia.org/wiki/CIELAB_color_space#Cylindrical_representation:_CIELCh_or_CIEHLC",
            slider_names: &["Lightness", "Chroma", "Hue"],
        }
    }

    fn init_sliders(&self) -> Box<[Slider<f32>]> {
        vec![
            Slider::new_linear("L", 50., 0.0, 100.0),
            Slider::new_linear("C", 64., 0.0, 128.0),
            Slider::new_linear("H", 180., 0.0, 360.0),
        ]
        .into_boxed_slice()
    }

    fn as_srgb(&self, sliders: &[Slider<f32>]) -> Srgb {
        let lch = Lch::new(sliders[0].value, sliders[1].value, sliders[2].value);
        lch.into_color()
    }

    fn compute_xyz(&self, sliders: &[Slider<f32>]) -> Xyz {
        let lch = Lch::new(sliders[0].value, sliders[1].value, sliders[2].value);
        lch.into_color()
    }

    fn compute_lab(&self, sliders: &[Slider<f32>]) -> Lab {
        let lch = Lch::new(sliders[0].value, sliders[1].value, sliders[2].value);
        lch.into_color()
    }

    fn from_srgb(srgb: Srgb) -> Box<[Slider<f32>]> {
        let lch: Lch = srgb.into_color();
        vec![
            Slider::new_linear("L", lch.l, 0.0, 100.0),
            Slider::new_linear("C", lch.chroma, 0.0, 128.0),
            Slider::new_linear("H", lch.hue.into_positive_degrees(), 0.0, 360.0),
        ]
        .into_boxed_slice()
    }
}
