use std::{backtrace, sync::RwLock};

use color_recall::game::{
    chooser_convert, ColorChallenge, ColorChooser, ExcludeReason, HSLChooser, HSVChooser,
    LABChooser, RGBChooser, Slider, XYZChooser,
};
use palette::Srgb;
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);

    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

pub struct GameContext {
    game: ColorChallenge,

    slider_srgb: (RGBChooser, Box<[Slider<f32>]>),
    slider_hsv: (HSVChooser, Box<[Slider<f32>]>),
    slider_hsl: (HSLChooser, Box<[Slider<f32>]>),
    slider_lab: (LABChooser, Box<[Slider<f32>]>),
    slider_xyz: (XYZChooser, Box<[Slider<f32>]>),
}

fn srgb_to_css(input: &Srgb) -> String {
    format!(
        "rgb({}, {}, {})",
        (input.red * 255.0) as u8,
        (input.green * 255.0) as u8,
        (input.blue * 255.0) as u8
    )
}

pub struct JSSliderInfo {
    pub name: &'static str,
    pub min: f32,
    pub max: f32,
    pub value: f32,
}

impl From<Slider<f32>> for JSSliderInfo {
    fn from(slider: Slider<f32>) -> Self {
        Self {
            name: slider.name,
            min: slider.min,
            max: slider.max,
            value: slider.value,
        }
    }
}

impl Into<JsValue> for JSSliderInfo {
    fn into(self) -> JsValue {
        let obj = js_sys::Object::new();

        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("name"),
            &JsValue::from_str(self.name),
        )
        .unwrap();

        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("min"),
            &JsValue::from_f64(self.min as f64),
        )
        .unwrap();

        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("max"),
            &JsValue::from_f64(self.max as f64),
        )
        .unwrap();

        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("value"),
            &JsValue::from_f64(self.value as f64),
        )
        .unwrap();

        obj.into()
    }
}

impl GameContext {
    pub fn new() -> Self {
        let srgb_chooser = RGBChooser::default();
        let srgb_sliders = srgb_chooser.init_sliders();
        Self {
            game: ColorChallenge::new(&mut OsRng::default()),
            slider_srgb: (srgb_chooser, srgb_sliders),
            slider_hsv: (HSVChooser::default(), HSVChooser::default().init_sliders()),
            slider_hsl: (HSLChooser::default(), HSLChooser::default().init_sliders()),
            slider_lab: (LABChooser::default(), LABChooser::default().init_sliders()),
            slider_xyz: (XYZChooser::default(), XYZChooser::default().init_sliders()),
        }
    }

    pub fn target_color_css(&self) -> String {
        srgb_to_css(&self.game.target_color())
    }

    pub fn current_color_css(&self, model: &str) -> String {
        let current_color = match model {
            "srgb" => self.slider_srgb.0.as_srgb(&self.slider_srgb.1),
            "hsv" => self.slider_hsv.0.as_srgb(&self.slider_hsv.1),
            "hsl" => self.slider_hsl.0.as_srgb(&self.slider_hsl.1),
            "lab" => self.slider_lab.0.as_srgb(&self.slider_lab.1),
            "xyz" => self.slider_xyz.0.as_srgb(&self.slider_xyz.1),
            _ => Srgb::new(0.0, 0.0, 0.0),
        };

        srgb_to_css(&current_color)
    }

    pub fn available_models(&self) -> Vec<String> {
        ["srgb", "hsv", "hsl", "lab", "xyz"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn model_name(&self, model: &str) -> String {
        match model {
            "srgb" => "sRGB",
            "hsv" => "HSV",
            "hsl" => "HSL",
            "lab" => "CIELAB",
            "xyz" => "CIEXYZ",
            _ => "Unknown",
        }
        .to_string()
    }

    pub fn model_info_link(&self, model: &str) -> String {
        match model {
            "srgb" => "https://en.wikipedia.org/wiki/SRGB",
            "rgb8" => "https://en.wikipedia.org/wiki/RGB_color_model",
            "hsv" => "https://en.wikipedia.org/wiki/HSL_and_HSV",
            "hsl" => "https://en.wikipedia.org/wiki/HSL_and_HSV",
            "lab" => "https://en.wikipedia.org/wiki/CIELAB_color_space",
            "xyz" => "https://en.wikipedia.org/wiki/CIE_1931_color_space",
            _ => "",
        }
        .to_string()
    }

    pub fn model_sliders(&self, model: &str) -> Option<Vec<JsValue>> {
        macro_rules! impl_model {
            ($name:literal, $slider:ident) => {
                if model == $name {
                    return Some(
                        self.$slider
                            .1
                            .iter()
                            .map(|s| {
                                let info: JSSliderInfo = s.clone().into();

                                info.into()
                            })
                            .collect(),
                    );
                }
            };
        }
        impl_model!("srgb", slider_srgb);
        impl_model!("hsv", slider_hsv);
        impl_model!("hsl", slider_hsl);
        impl_model!("lab", slider_lab);
        impl_model!("xyz", slider_xyz);
        None
    }

    pub fn switch_model(&mut self, reference: &str) {
        macro_rules! cross_propagate_one {
            ($from:ident, $to:ident) => {
                chooser_convert(&self.$from.0, &self.$to.0, &self.$from.1, &mut self.$to.1);
            };
        }
        macro_rules! cross_propagate {
            ($from:ident => $($to:ident),+) => {
                $(cross_propagate_one!($from, $to);)+
            };
        }

        match reference {
            "srgb" => {
                cross_propagate!(slider_srgb =>  slider_hsv, slider_hsl, slider_lab, slider_xyz);
            }
            "hsv" => {
                cross_propagate!(slider_hsv => slider_srgb, slider_hsl, slider_lab, slider_xyz);
            }
            "hsl" => {
                cross_propagate!(slider_hsl => slider_srgb, slider_hsv, slider_lab, slider_xyz);
            }
            "lab" => {
                cross_propagate!(slider_lab => slider_srgb, slider_hsv, slider_hsl, slider_xyz);
            }
            "xyz" => {
                cross_propagate!(slider_xyz => slider_srgb, slider_hsv, slider_hsl, slider_lab);
            }
            _ => {}
        }
    }

    pub fn update_slider(&mut self, model: &str, values: &[f32]) {
        macro_rules! cross_propagate_one {
            ($from:ident, $to:ident) => {
                chooser_convert(&self.$from.0, &self.$to.0, &self.$from.1, &mut self.$to.1);
            };
        }
        match model {
            "srgb" => {
                self.slider_srgb
                    .1
                    .iter_mut()
                    .zip(values.iter())
                    .for_each(|(s, v)| {
                        s.value = *v;
                    });
            }
            "hsv" => {
                self.slider_hsv
                    .1
                    .iter_mut()
                    .zip(values.iter())
                    .for_each(|(s, v)| {
                        s.value = *v;
                    });
                cross_propagate_one!(slider_hsv, slider_srgb);
            }
            "hsl" => {
                self.slider_hsl.1 = values
                    .iter()
                    .zip(self.slider_hsl.1.iter())
                    .map(|(v, s)| {
                        let mut s = s.clone();
                        s.value = *v as f32;
                        s
                    })
                    .collect();
                cross_propagate_one!(slider_hsl, slider_srgb);
            }
            "lab" => {
                self.slider_lab
                    .1
                    .iter_mut()
                    .zip(values.iter())
                    .for_each(|(s, v)| {
                        s.value = *v;
                    });
                cross_propagate_one!(slider_lab, slider_srgb);
            }
            "xyz" => {
                self.slider_xyz
                    .1
                    .iter_mut()
                    .zip(values.iter())
                    .for_each(|(s, v)| {
                        s.value = *v;
                    });
                cross_propagate_one!(slider_xyz, slider_srgb);
            }
            _ => {}
        }
    }

    pub fn compute_score(&self) -> f32 {
        self.game
            .compute_distance(self.slider_srgb.0.compute_lab(&self.slider_srgb.1))
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(debug_assertions)]
    {
        std::panic::set_hook(Box::new(|info| {
            log(&format!("Panic: {:?}", info).into());
            log(&format!("Payload: {:?}", info.payload()).into());
            log(&format!("Backtrace: {:?}", backtrace::Backtrace::capture()).into());

            error(&format!("Panic: {:?}", info));
        }));
    }
}

static GAME_CONTEXT: RwLock<Option<GameContext>> = RwLock::new(None);

#[wasm_bindgen]
pub fn init_game() {
    let mut game = GAME_CONTEXT.write().unwrap();
    *game = Some(GameContext::new());
}

#[wasm_bindgen]
pub fn color_acceptable() -> Option<String> {
    let game = GAME_CONTEXT.read().unwrap();
    let srgb = &game.as_ref().unwrap().slider_srgb;
    match ColorChallenge::is_excluded(&srgb.0.as_srgb(&srgb.1)) {
        None => None,
        Some(ExcludeReason::LowSaturation) => Some("low_saturation".to_string()),
        Some(ExcludeReason::TooBright) => Some("too_bright".to_string()),
        Some(ExcludeReason::TooDark) => Some("too_dark".to_string()),
    }
}

#[wasm_bindgen]
pub fn target_color_css() -> String {
    GAME_CONTEXT
        .read()
        .unwrap()
        .as_ref()
        .unwrap()
        .target_color_css()
}

#[wasm_bindgen]
pub fn current_color_css(model: &str) -> String {
    GAME_CONTEXT
        .read()
        .unwrap()
        .as_ref()
        .unwrap()
        .current_color_css(model)
}

#[wasm_bindgen]
pub fn available_models() -> Vec<String> {
    GAME_CONTEXT
        .read()
        .unwrap()
        .as_ref()
        .unwrap()
        .available_models()
}

#[wasm_bindgen]
pub fn model_name(model: &str) -> String {
    GAME_CONTEXT
        .read()
        .unwrap()
        .as_ref()
        .unwrap()
        .model_name(model)
}

#[wasm_bindgen]
pub fn model_info_link(model: &str) -> String {
    GAME_CONTEXT
        .read()
        .unwrap()
        .as_ref()
        .unwrap()
        .model_info_link(model)
}

#[wasm_bindgen]
pub fn model_sliders(model: &str) -> Option<Vec<JsValue>> {
    GAME_CONTEXT
        .read()
        .unwrap()
        .as_ref()
        .unwrap()
        .model_sliders(model)
}

#[wasm_bindgen]
pub fn switch_model(reference: &str) {
    GAME_CONTEXT
        .write()
        .unwrap()
        .as_mut()
        .unwrap()
        .switch_model(reference);
}

#[wasm_bindgen]
pub fn update_slider(model: &str, values: &[f32]) {
    GAME_CONTEXT
        .write()
        .unwrap()
        .as_mut()
        .unwrap()
        .update_slider(model, values);
}

#[wasm_bindgen]
pub fn compute_score() -> f32 {
    GAME_CONTEXT
        .read()
        .unwrap()
        .as_ref()
        .unwrap()
        .compute_score()
}
