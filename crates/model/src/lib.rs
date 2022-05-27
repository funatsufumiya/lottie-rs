use std::str::FromStr;

pub use euclid::default::Rect;
pub use euclid::rect;
use serde::{Deserialize, Serialize};
pub use serde_json::Error;
pub type Vector2D = euclid::default::Vector2D<f32>;

mod helpers;

use helpers::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "v", default)]
    version: Option<String>,
    #[serde(rename = "ip")]
    pub start_frame: f32,
    #[serde(rename = "op")]
    pub end_frame: f32,
    #[serde(rename = "fr")]
    pub frame_rate: f32,
    #[serde(rename = "w")]
    pub width: u32,
    #[serde(rename = "h")]
    pub height: u32,
    pub layers: Vec<Layer>,
    #[serde(default)]
    pub assets: Vec<Precomposition>,
    #[serde(default)]
    pub fonts: FontList,
}

impl Model {
    pub fn from_reader<R: std::io::Read>(r: R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(r)
    }

    pub fn duration(&self) -> f32 {
        (self.end_frame - self.start_frame) as f32 / self.frame_rate as f32
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Layer {
    #[serde(
        deserialize_with = "bool_from_int",
        serialize_with = "int_from_bool",
        rename = "ddd",
        default
    )]
    is_3d: bool,
    #[serde(rename = "hd", default)]
    pub hidden: bool,
    #[serde(rename = "ind", default)]
    pub index: Option<u32>,
    #[serde(rename = "parent", default)]
    pub parent_index: Option<u32>,
    #[serde(skip)]
    pub id: u32,
    #[serde(
        rename = "ao",
        deserialize_with = "bool_from_int",
        serialize_with = "int_from_bool",
        default
    )]
    pub auto_orient: bool,
    #[serde(rename = "ip")]
    pub start_frame: f32,
    #[serde(rename = "op")]
    pub end_frame: f32,
    #[serde(rename = "st")]
    pub start_time: f32,
    #[serde(rename = "nm")]
    name: Option<String>,
    #[serde(rename = "ks", default)]
    pub transform: Option<Transform>,
    #[serde(flatten)]
    pub content: LayerContent,
}

#[derive(Debug, Clone)]
pub enum LayerContent {
    Precomposition(PreCompositionRef),
    SolidColor {
        color: Rgba,
        height: f32,
        width: f32,
    },
    Image,
    Empty,
    Shape(ShapeGroup),
    Text,
    Audio,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreCompositionRef {
    #[serde(rename = "refId")]
    pub ref_id: String,
    #[serde(rename = "w")]
    width: u32,
    #[serde(rename = "h")]
    height: u32,
    #[serde(rename = "tm")]
    time_remapping: Option<Animated<f32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transform {
    #[serde(rename = "a", default)]
    pub anchor: Option<Animated<Vector2D>>,
    #[serde(rename = "p", default)]
    pub position: Option<Animated<Vector2D>>,
    #[serde(rename = "s", default = "default_vec2_100")]
    pub scale: Animated<Vector2D>,
    #[serde(rename = "r", default)]
    pub rotation: Animated<f32>,
    #[serde(skip)]
    pub auto_orient: bool,
    #[serde(rename = "o", default = "default_number_100")]
    pub opacity: Animated<f32>,
    #[serde(rename = "sk", default)]
    skew: Option<Animated<Vector2D>>,
    #[serde(rename = "sa", default)]
    skew_axis: Option<Animated<Vector2D>>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            anchor: Default::default(),
            position: Default::default(),
            scale: default_vec2_100(),
            rotation: Default::default(),
            opacity: default_number_100(),
            skew: Default::default(),
            skew_axis: Default::default(),
            auto_orient: false,
        }
    }
}

impl Transform {
    pub fn is_identity(&self) -> bool {
        false
        // TODO:
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepeaterTransform {
    #[serde(rename = "a", default)]
    anchor: Animated<Vector2D>,
    #[serde(rename = "p")]
    position: Animated<Vector2D>,
    #[serde(rename = "s")]
    scale: Animated<Vector2D>,
    #[serde(rename = "r")]
    rotation: Animated<f32>,
    #[serde(rename = "so")]
    start_opacity: Animated<f32>,
    #[serde(rename = "eo")]
    end_opacity: Animated<f32>,
    #[serde(rename = "sk", default)]
    skew: Option<Animated<Vector2D>>,
    #[serde(rename = "sa", default)]
    skew_axis: Option<Animated<Vector2D>>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct KeyFrame<T> {
    #[serde(rename = "s")]
    pub start_value: T,
    #[serde(skip)]
    pub end_value: T,
    #[serde(rename = "t", default)]
    pub start_frame: f32,
    #[serde(skip)]
    pub end_frame: f32,
    #[serde(rename = "o", default)]
    pub easing_out: Option<Easing>,
    #[serde(rename = "i", default)]
    pub easing_in: Option<Easing>,
}

impl<T: Clone> KeyFrame<T> {
    pub fn from_value(value: T) -> Self {
        KeyFrame {
            start_value: value.clone(),
            end_value: value,
            start_frame: 0.0,
            end_frame: 0.0,
            easing_out: None,
            easing_in: None,
        }
    }

    pub fn alter_value<U>(&self, start: U, end: U) -> KeyFrame<U> {
        KeyFrame {
            start_value: start,
            end_value: end,
            start_frame: self.start_frame,
            end_frame: self.end_frame,
            easing_out: self.easing_out.clone(),
            easing_in: self.easing_in.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Easing {
    #[serde(deserialize_with = "array_from_array_or_number")]
    pub x: Vec<f32>,
    #[serde(deserialize_with = "array_from_array_or_number")]
    pub y: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Animated<T> {
    #[serde(
        deserialize_with = "bool_from_int",
        serialize_with = "int_from_bool",
        rename = "a",
        default
    )]
    pub animated: bool,
    #[serde(
        deserialize_with = "keyframes_from_array",
        serialize_with = "array_from_keyframes",
        bound = "T: FromTo<helpers::Value>",
        rename = "k"
    )]
    pub keyframes: Vec<KeyFrame<T>>,
}

impl<T> Default for Animated<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            animated: false,
            keyframes: vec![KeyFrame::default()],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FontList {
    pub list: Vec<Font>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Font {
    #[serde(default)]
    ascent: Option<f32>,
    #[serde(rename = "fFamily")]
    family: String,
    #[serde(rename = "fName")]
    name: String,
    #[serde(rename = "fStyle")]
    style: String,
    #[serde(rename = "fPath", default)]
    path: Option<String>,
    #[serde(rename = "fWeight")]
    weight: Option<String>,
    #[serde(default)]
    origin: FontPathOrigin,
    #[serde(rename = "fClass", default)]
    class: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl FromStr for Rgba {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl ToString for Rgba {
    fn to_string(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new_f32(r: f32, g: f32, b: f32) -> Rgb {
        Rgb {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
        }
    }

    pub fn new_u8(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r, g, b }
    }
}

#[derive(Debug, Clone)]
pub struct AnimatedColorList {
    animated: bool,
    colors: Vec<Rgba>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShapeLayer {
    #[serde(rename = "nm", default)]
    name: Option<String>,
    #[serde(rename = "hd", default)]
    pub hidden: bool,
    #[serde(skip)]
    pub id: u32,
    #[serde(flatten)]
    pub shape: Shape,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "ty")]
pub enum Shape {
    #[serde(rename = "rc")]
    Rectangle(Rectangle),
    #[serde(rename = "el")]
    Ellipse(Ellipse),
    #[serde(rename = "sr")]
    PolyStar(PolyStar),
    #[serde(rename = "sh")]
    Path {
        #[serde(rename = "ks")]
        d: Animated<Vec<Bezier>>,
    },
    #[serde(rename = "fl")]
    Fill(Fill),
    #[serde(rename = "st")]
    Stroke(Stroke),
    #[serde(rename = "gf")]
    GradientFill {
        #[serde(rename = "o")]
        opacity: Animated<f32>,
        #[serde(rename = "r")]
        fill_rule: FillRule,
        #[serde(rename = "s")]
        start: Animated<Vector2D>,
        #[serde(rename = "e")]
        end: Animated<Vector2D>,
        #[serde(rename = "t")]
        gradient_ty: GradientType,
        #[serde(rename = "g")]
        colors: AnimatedColorList,
    },
    #[serde(rename = "gs")]
    GradientStroke {
        #[serde(rename = "lc")]
        line_cap: LineCap,
        #[serde(rename = "lj")]
        line_join: LineJoin,
        #[serde(rename = "ml")]
        miter_limit: f32,
        #[serde(rename = "o")]
        opacity: Animated<f32>,
        #[serde(rename = "w")]
        width: Animated<f32>,
        #[serde(rename = "d", default)]
        dashes: Vec<StrokeDash>,
        #[serde(rename = "s")]
        start: Animated<Vector2D>,
        #[serde(rename = "e")]
        end: Animated<Vector2D>,
        #[serde(rename = "t")]
        gradient_ty: GradientType,
        #[serde(rename = "g")]
        colors: AnimatedColorList,
    },
    #[serde(rename = "gr")]
    Group {
        // TODO: add np property
        #[serde(rename = "it")]
        shapes: Vec<ShapeLayer>,
    },
    #[serde(rename = "tr")]
    Transform(Transform),
    #[serde(rename = "rp")]
    Repeater {
        #[serde(rename = "c")]
        copies: Animated<f32>,
        #[serde(rename = "o")]
        offset: Animated<f32>,
        #[serde(rename = "m")]
        composite: Composite,
        #[serde(rename = "tr")]
        transform: RepeaterTransform,
    },
    #[serde(rename = "tm")]
    Trim {
        #[serde(rename = "s")]
        start: Animated<f32>,
        #[serde(rename = "e")]
        end: Animated<f32>,
        #[serde(rename = "o")]
        offset: Animated<f32>,
        #[serde(rename = "m")]
        multiple_shape: TrimMultipleShape,
    },
    #[serde(rename = "rd")]
    RoundedCorners {
        #[serde(rename = "r")]
        radius: Animated<f32>,
    },
    #[serde(rename = "pb")]
    PuckerBloat {
        #[serde(rename = "a")]
        amount: Animated<f32>,
    },
    #[serde(rename = "tw")]
    Twist {
        #[serde(rename = "a")]
        angle: Animated<f32>,
        #[serde(rename = "c")]
        center: Animated<Vector2D>,
    },
    #[serde(rename = "mm")]
    Merge {
        #[serde(rename = "mm")]
        mode: MergeMode,
    },
    #[serde(rename = "op")]
    OffsetPath {
        #[serde(rename = "a")]
        amount: Animated<f32>,
        #[serde(rename = "lj")]
        line_join: LineJoin,
        #[serde(rename = "ml")]
        miter_limit: f32,
    },
    #[serde(rename = "zz")]
    ZigZag {
        #[serde(rename = "r")]
        radius: Animated<f32>,
        #[serde(rename = "s")]
        distance: Animated<f32>,
        #[serde(rename = "pt")]
        ridges: Animated<f32>,
    },
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum PolyStarType {
    Star = 1,
    Polygon = 2,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum FillRule {
    NonZero = 1,
    EvenOdd = 2,
}

impl Default for FillRule {
    fn default() -> Self {
        FillRule::NonZero
    }
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum LineCap {
    Butt = 1,
    Round = 2,
    Square = 3,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum LineJoin {
    Miter = 1,
    Round = 2,
    Bevel = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StrokeDash {
    #[serde(rename = "v")]
    length: Animated<f32>,
    #[serde(rename = "n")]
    ty: StrokeDashType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum StrokeDashType {
    #[serde(rename = "d")]
    Dash,
    #[serde(rename = "g")]
    Gap,
    #[serde(rename = "o")]
    Offset,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum GradientType {
    Linear = 1,
    Radial = 2,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Composite {
    Above = 1,
    Below = 2,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum TrimMultipleShape {
    Individually = 1,
    Simultaneously = 2,
}

#[derive(
    serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone, Copy, PartialEq,
)]
#[repr(u8)]
pub enum ShapeDirection {
    Clockwise = 1,
    CounterClockwise = 2,
}

impl Default for ShapeDirection {
    fn default() -> Self {
        ShapeDirection::Clockwise
    }
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum MergeMode {
    #[serde(other)]
    Unsupported = 1,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum FontPathOrigin {
    Local = 0,
    CssUrl = 1,
    ScriptUrl = 2,
    FontUrl = 3,
}

impl Default for FontPathOrigin {
    fn default() -> Self {
        FontPathOrigin::Local
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fill {
    #[serde(rename = "o")]
    pub opacity: Animated<f32>,
    #[serde(rename = "c")]
    pub color: Animated<Rgb>,
    #[serde(rename = "r", default)]
    pub fill_rule: FillRule,
}

impl Fill {
    pub fn transparent() -> Fill {
        Fill {
            opacity: Animated {
                animated: false,
                keyframes: vec![KeyFrame::from_value(0.0)],
            },
            color: Animated {
                animated: false,
                keyframes: vec![KeyFrame::from_value(Rgb::new_u8(0, 0, 0))],
            },
            fill_rule: FillRule::NonZero,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stroke {
    #[serde(rename = "lc")]
    pub line_cap: LineCap,
    #[serde(rename = "lj")]
    pub line_join: LineJoin,
    #[serde(rename = "ml")]
    miter_limit: f32,
    #[serde(rename = "o")]
    pub opacity: Animated<f32>,
    #[serde(rename = "w")]
    pub width: Animated<f32>,
    #[serde(rename = "d", default)]
    dashes: Vec<StrokeDash>,
    #[serde(rename = "c")]
    pub color: Animated<Rgb>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rectangle {
    #[serde(rename = "d", default)]
    pub direction: ShapeDirection,
    #[serde(rename = "p")]
    pub position: Animated<Vector2D>,
    #[serde(rename = "s")]
    pub size: Animated<Vector2D>,
    #[serde(rename = "r")]
    pub radius: Animated<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ellipse {
    #[serde(rename = "d", default)]
    pub direction: ShapeDirection,
    #[serde(rename = "p")]
    pub position: Animated<Vector2D>,
    #[serde(rename = "s")]
    pub size: Animated<Vector2D>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolyStar {
    #[serde(rename = "d", default)]
    pub direction: ShapeDirection,
    #[serde(rename = "p")]
    pub position: Animated<Vector2D>,
    #[serde(rename = "or")]
    pub outer_radius: Animated<f32>,
    #[serde(rename = "os")]
    pub outer_roundness: Animated<f32>,
    #[serde(rename = "ir")]
    pub inner_radius: Animated<f32>,
    #[serde(rename = "is")]
    pub inner_roundness: Animated<f32>,
    #[serde(rename = "r")]
    pub rotation: Animated<f32>,
    #[serde(rename = "pt")]
    pub points: Animated<f32>,
    #[serde(rename = "sy")]
    pub star_type: PolyStarType,
}

pub enum Assets {
    Image,
    Sound,
    Precomposition(Precomposition),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Precomposition {
    pub id: String,
    pub layers: Vec<Layer>,
    #[serde(rename = "nm")]
    name: Option<String>,
    #[serde(rename = "fr")]
    pub frame_rate: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShapeGroup {
    pub shapes: Vec<ShapeLayer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bezier {
    #[serde(rename = "c", default)]
    pub closed: bool,
    #[serde(
        rename = "v",
        deserialize_with = "vec_from_array",
        serialize_with = "array_from_vec"
    )]
    pub verticies: Vec<Vector2D>,
    #[serde(
        rename = "i",
        deserialize_with = "vec_from_array",
        serialize_with = "array_from_vec"
    )]
    pub in_tangent: Vec<Vector2D>,
    #[serde(
        rename = "o",
        deserialize_with = "vec_from_array",
        serialize_with = "array_from_vec"
    )]
    pub out_tangent: Vec<Vector2D>,
}
