use std::cmp::{Eq, PartialEq};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use crate::Math::Unstable::clamp;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new() -> Color {
        Color { r: 255.0, g: 255.0, b: 255.0, a: 255.0 }
	}

	pub const fn from(r: u8, g: u8, b: u8, a: u8) -> Color {
		Color { r: r as f32, g: g as f32, b: b as f32, a: a as f32 }
	}

	pub fn from_unit_interval(r: f32, g: f32, b: f32, a: f32) -> Self {
		assert!(r >= 0.0 && g >= 0.0 && b >= 0.0 && a >= 0.0);
		assert!(r <= 1.0 && g <= 1.0 && b <= 1.0 && a <= 1.0);
        Color { r: r * 255.0, g: g * 255.0, b: b * 255.0, a: a * 255.0 }
	}
	
	pub fn set(&mut self, r: f32, g: f32, b: f32, a: f32) {
		assert!(r >= 0.0 && g >= 0.0 && b >= 0.0 && a >= 0.0);
		assert!(r <= 255.0 && g <= 255.0 && b <= 255.0 && a <= 255.0);
		self.r = r;
		self.g = g;
		self.b = b;
		self.a = a;
	}

	pub fn unit_interval(self) -> (f32, f32, f32, f32) {
		(self.r / 255.0, self.g / 255.0, self.b / 255.0, self.a / 255.0)
	}

	pub const fn rgb(self) -> (f32, f32, f32) {
		(self.r, self.g, self.b)
	}

	pub const fn rgba(self) -> (f32, f32, f32, f32) {
		(self.r, self.g, self.b, self.a)
	}
}

impl Eq for Color {}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}

impl Add for Color {
	type Output = Color;

	fn add(self, other: Color) -> Color {
		Color {
			r: clamp(self.r + other.r, 0.0, 255.0),
			g: clamp(self.g + other.g, 0.0, 255.0),
			b: clamp(self.b + other.b, 0.0, 255.0),
			a: clamp(self.a + other.a, 0.0, 255.0),
		}
	}
}

impl AddAssign for Color {
	fn add_assign(&mut self, other: Color) {
		self.r = clamp(self.r + other.r, 0.0, 255.0);
		self.g = clamp(self.g + other.g, 0.0, 255.0);
		self.b = clamp(self.b + other.b, 0.0, 255.0);
		self.a = clamp(self.a + other.a, 0.0, 255.0);
	}
}

impl Sub for Color {
	type Output = Color;

	fn sub(self, other: Color) -> Color {
		Color {
			r: clamp(self.r - other.r, 0.0, 255.0),
			g: clamp(self.g - other.g, 0.0, 255.0),
			b: clamp(self.b - other.b, 0.0, 255.0),
			a: clamp(self.a - other.a, 0.0, 255.0),
		}
	}
}

impl SubAssign for Color {
	fn sub_assign(&mut self, other: Color) {
		self.r = clamp(self.r - other.r, 0.0, 255.0);
		self.g = clamp(self.g - other.g, 0.0, 255.0);
		self.b = clamp(self.b - other.b, 0.0, 255.0);
		self.a = clamp(self.a - other.a, 0.0, 255.0);
	}
}

impl Mul for Color {
	type Output = Color;

	fn mul(self, other: Color) -> Color {
		Color {
			r: clamp(self.r * other.r, 0.0, 255.0),
			g: clamp(self.g * other.g, 0.0, 255.0),
			b: clamp(self.b * other.b, 0.0, 255.0),
			a: clamp(self.a * other.a, 0.0, 255.0),
		}
	}
}

impl MulAssign for Color {
	fn mul_assign(&mut self, other: Color) {
		self.r = clamp(self.r * other.r, 0.0, 255.0);
		self.g = clamp(self.g * other.g, 0.0, 255.0);
		self.b = clamp(self.b * other.b, 0.0, 255.0);
		self.a = clamp(self.a * other.a, 0.0, 255.0);
	}
}

impl <T> Mul<T> for Color
	where T: Into<f32> {
	type Output = Color;

	fn mul(self, value: T) -> Color {
		let scale = value.into();

		Color {
			r: clamp(self.r * scale, 0.0, 255.0),
			g: clamp(self.g * scale, 0.0, 255.0),
			b: clamp(self.b * scale, 0.0, 255.0),
			a: clamp(self.a * scale, 0.0, 255.0),
		}
	}
}

impl <T> MulAssign<T> for Color
	where T: Into<f32> {
	fn mul_assign(&mut self, value: T) {
		let scale = value.into();

		self.r = clamp(self.r * scale, 0.0, 255.0);
		self.g = clamp(self.g * scale, 0.0, 255.0);
		self.b = clamp(self.b * scale, 0.0, 255.0);
		self.a = clamp(self.a * scale, 0.0, 255.0);
	}
}

impl Div for Color {
	type Output = Color;

	fn div(self, other: Color) -> Color {
		Color {
			r: clamp(self.r / other.r, 0.0, 255.0),
			g: clamp(self.g / other.g, 0.0, 255.0),
			b: clamp(self.b / other.b, 0.0, 255.0),
			a: clamp(self.a / other.a, 0.0, 255.0),
		}
	}
}

impl DivAssign for Color {
	fn div_assign(&mut self, other: Color) {
		self.r = clamp(self.r / other.r, 0.0, 255.0);
		self.g = clamp(self.g / other.g, 0.0, 255.0);
		self.b = clamp(self.b / other.b, 0.0, 255.0);
		self.a = clamp(self.a / other.a, 0.0, 255.0);
	}
}

impl <T> Div<T> for Color
	where T: Into<f32> {
	type Output = Color;

	fn div(self, value: T) -> Color {
		let scale = value.into();

		Color {
			r: clamp(self.r / scale, 0.0, 255.0),
			g: clamp(self.g / scale, 0.0, 255.0),
			b: clamp(self.b / scale, 0.0, 255.0),
			a: clamp(self.a / scale, 0.0, 255.0),
		}
	}
}

impl <T> DivAssign<T> for Color
	where T: Into<f32> {
	fn div_assign(&mut self, value: T) {
		let scale = value.into();

		self.r = clamp(self.r / scale, 0.0, 255.0);
		self.g = clamp(self.g / scale, 0.0, 255.0);
		self.b = clamp(self.b / scale, 0.0, 255.0);
		self.a = clamp(self.a / scale, 0.0, 255.0);
	}
}

impl Color {
	pub const INDIANRED: Color = Color::from(205, 92, 92, 255);
	pub const LIGHTCORAL: Color = Color::from(240, 128, 128, 255);
	pub const SALMON: Color = Color::from(250, 128, 114, 255);
	pub const DARKSALMON: Color = Color::from(233, 150, 122, 255);
	pub const LIGHTSALMON: Color = Color::from(255, 160, 122, 255);
	pub const CRIMSON: Color = Color::from(220, 20, 60, 255);
	pub const RED: Color = Color::from(255, 0, 0, 255);
	pub const FIREBRICK: Color = Color::from(178, 34, 34, 255);
	pub const DARKRED: Color = Color::from(139, 0, 0, 255);
	pub const PINK: Color = Color::from(255, 192, 203, 255);
	pub const LIGHTPINK: Color = Color::from(255, 182, 193, 255);
	pub const HOTPINK: Color = Color::from(255, 105, 180, 255);
	pub const DEEPPINK: Color = Color::from(255, 20, 147, 255);
	pub const MEDIUMVIOLETRED: Color = Color::from(199, 21, 133, 255);
	pub const PALEVIOLETRED: Color = Color::from(219, 112, 147, 255);
	pub const CORAL: Color = Color::from(255, 127, 80, 255);
	pub const TOMATO: Color = Color::from(255, 99, 71, 255);
	pub const ORANGERED: Color = Color::from(255, 69, 0, 255);
	pub const DARKORANGE: Color = Color::from(255, 140, 0, 255);
	pub const ORANGE: Color = Color::from(255, 165, 0, 255);
	pub const GOLD: Color = Color::from(255, 215, 0, 255);
	pub const YELLOW: Color = Color::from(255, 255, 0, 255);
	pub const LIGHTYELLOW: Color = Color::from(255, 255, 224, 255);
	pub const LEMONCHIFFON: Color = Color::from(255, 250, 205, 255);
	pub const LIGHTGOLDENRODYELLOW: Color = Color::from(250, 250, 210, 255);
	pub const PAPAYAWHIP: Color = Color::from(255, 239, 213, 255);
	pub const MOCCASIN: Color = Color::from(255, 228, 181, 255);
	pub const PEACHPUFF: Color = Color::from(255, 218, 185, 255);
	pub const PALEGOLDENROD: Color = Color::from(238, 232, 170, 255);
	pub const KHAKI: Color = Color::from(240, 230, 140, 255);
	pub const DARKKHAKI: Color = Color::from(189, 183, 107, 255);
	pub const LAVENDER: Color = Color::from(230, 230, 250, 255);
	pub const THISTLE: Color = Color::from(216, 191, 216, 255);
	pub const PLUM: Color = Color::from(221, 160, 221, 255);
	pub const VIOLET: Color = Color::from(238, 130, 238, 255);
	pub const ORCHID: Color = Color::from(218, 112, 214, 255);
	pub const FUCHSIA: Color = Color::from(255, 0, 255, 255);
	pub const MAGENTA: Color = Color::from(255, 0, 255, 255);
	pub const MEDIUMORCHID: Color = Color::from(186, 85, 211, 255);
	pub const MEDIUMPURPLE: Color = Color::from(147, 112, 219, 255);
	pub const REBECCAPURPLE: Color = Color::from(102, 51, 153, 255);
	pub const BLUEVIOLET: Color = Color::from(138, 43, 226, 255);
	pub const DARKVIOLET: Color = Color::from(148, 0, 211, 255);
	pub const DARKORCHID: Color = Color::from(153, 50, 204, 255);
	pub const DARKMAGENTA: Color = Color::from(139, 0, 139, 255);
	pub const PURPLE: Color = Color::from(128, 0, 128, 255);
	pub const INDIGO: Color = Color::from(75, 0, 130, 255);
	pub const SLATEBLUE: Color = Color::from(106, 90, 205, 255);
	pub const DARKSLATEBLUE: Color = Color::from(72, 61, 139, 255);
	pub const GREENYELLOW: Color = Color::from(173, 255, 47, 255);
	pub const CHARTREUSE: Color = Color::from(127, 255, 0, 255);
	pub const LAWNGREEN: Color = Color::from(124, 252, 0, 255);
	pub const LIME: Color = Color::from(0, 255, 0, 255);
	pub const LIMEGREEN: Color = Color::from(50, 205, 50, 255);
	pub const PALEGREEN: Color = Color::from(152, 251, 152, 255);
	pub const LIGHTGREEN: Color = Color::from(144, 238, 144, 255);
	pub const MEDIUMSPRINGGREEN: Color = Color::from(0, 250, 154, 255);
	pub const SPRINGGREEN: Color = Color::from(0, 255, 127, 255);
	pub const MEDIUMSEAGREEN: Color = Color::from(60, 179, 113, 255);
	pub const SEAGREEN: Color = Color::from(46, 139, 87, 255);
	pub const FORESTGREEN: Color = Color::from(34, 139, 34, 255);
	pub const GREEN: Color = Color::from(0, 128, 0, 255);
	pub const DARKGREEN: Color = Color::from(0, 100, 0, 255);
	pub const YELLOWGREEN: Color = Color::from(154, 205, 50, 255);
	pub const OLIVEDRAB: Color = Color::from(107, 142, 35, 255);
	pub const OLIVE: Color = Color::from(128, 128, 0, 255);
	pub const DARKOLIVEGREEN: Color = Color::from(85, 107, 47, 255);
	pub const MEDIUMAQUAMARINE: Color = Color::from(102, 205, 170, 255);
	pub const DARKSEAGREEN: Color = Color::from(143, 188, 139, 255);
	pub const LIGHTSEAGREEN: Color = Color::from(32, 178, 170, 255);
	pub const DARKCYAN: Color = Color::from(0, 139, 139, 255);
	pub const TEAL: Color = Color::from(0, 128, 128, 255);
	pub const AQUA: Color = Color::from(0, 255, 255, 255);
	pub const CYAN: Color = Color::from(0, 255, 255, 255);
	pub const LIGHTCYAN: Color = Color::from(224, 255, 255, 255);
	pub const PALETURQUOISE: Color = Color::from(175, 238, 238, 255);
	pub const AQUAMARINE: Color = Color::from(127, 255, 212, 255);
	pub const TURQUOISE: Color = Color::from(64, 224, 208, 255);
	pub const MEDIUMTURQUOISE: Color = Color::from(72, 209, 204, 255);
	pub const DARKTURQUOISE: Color = Color::from(0, 206, 209, 255);
	pub const CADETBLUE: Color = Color::from(95, 158, 160, 255);
	pub const STEELBLUE: Color = Color::from(70, 130, 180, 255);
	pub const LIGHTSTEELBLUE: Color = Color::from(176, 196, 222, 255);
	pub const POWDERBLUE: Color = Color::from(176, 224, 230, 255);
	pub const LIGHTBLUE: Color = Color::from(173, 216, 230, 255);
	pub const SKYBLUE: Color = Color::from(135, 206, 235, 255);
	pub const LIGHTSKYBLUE: Color = Color::from(135, 206, 250, 255);
	pub const DEEPSKYBLUE: Color = Color::from(0, 191, 255, 255);
	pub const DODGERBLUE: Color = Color::from(30, 144, 255, 255);
	pub const CORNFLOWERBLUE: Color = Color::from(100, 149, 237, 255);
	pub const MEDIUMSLATEBLUE: Color = Color::from(123, 104, 238, 255);
	pub const ROYALBLUE: Color = Color::from(65, 105, 225, 255);
	pub const BLUE: Color = Color::from(0, 0, 255, 255);
	pub const MEDIUMBLUE: Color = Color::from(0, 0, 205, 255);
	pub const DARKBLUE: Color = Color::from(0, 0, 139, 255);
	pub const NAVY: Color = Color::from(0, 0, 128, 255);
	pub const MIDNIGHTBLUE: Color = Color::from(25, 25, 112, 255);
	pub const CORNSILK: Color = Color::from(255, 248, 220, 255);
	pub const BLANCHEDALMOND: Color = Color::from(255, 235, 205, 255);
	pub const BISQUE: Color = Color::from(255, 228, 196, 255);
	pub const NAVAJOWHITE: Color = Color::from(255, 222, 173, 255);
	pub const WHEAT: Color = Color::from(245, 222, 179, 255);
	pub const BURLYWOOD: Color = Color::from(222, 184, 135, 255);
	pub const TAN: Color = Color::from(210, 180, 140, 255);
	pub const ROSYBROWN: Color = Color::from(188, 143, 143, 255);
	pub const SANDYBROWN: Color = Color::from(244, 164, 96, 255);
	pub const GOLDENROD: Color = Color::from(218, 165, 32, 255);
	pub const DARKGOLDENROD: Color = Color::from(184, 134, 11, 255);
	pub const PERU: Color = Color::from(205, 133, 63, 255);
	pub const CHOCOLATE: Color = Color::from(210, 105, 30, 255);
	pub const SADDLEBROWN: Color = Color::from(139, 69, 19, 255);
	pub const SIENNA: Color = Color::from(160, 82, 45, 255);
	pub const BROWN: Color = Color::from(165, 42, 42, 255);
	pub const MAROON: Color = Color::from(128, 0, 0, 255);
	pub const WHITE: Color = Color::from(255, 255, 255, 255);
	pub const SNOW: Color = Color::from(255, 250, 250, 255);
	pub const HONEYDEW: Color = Color::from(240, 255, 240, 255);
	pub const MINTCREAM: Color = Color::from(245, 255, 250, 255);
	pub const AZURE: Color = Color::from(240, 255, 255, 255);
	pub const ALICEBLUE: Color = Color::from(240, 248, 255, 255);
	pub const GHOSTWHITE: Color = Color::from(248, 248, 255, 255);
	pub const WHITESMOKE: Color = Color::from(245, 245, 245, 255);
	pub const SEASHELL: Color = Color::from(255, 245, 238, 255);
	pub const BEIGE: Color = Color::from(245, 245, 220, 255);
	pub const OLDLACE: Color = Color::from(253, 245, 230, 255);
	pub const FLORALWHITE: Color = Color::from(255, 250, 240, 255);
	pub const IVORY: Color = Color::from(255, 255, 240, 255);
	pub const ANTIQUEWHITE: Color = Color::from(250, 235, 215, 255);
	pub const LINEN: Color = Color::from(250, 240, 230, 255);
	pub const LAVENDERBLUSH: Color = Color::from(255, 240, 245, 255);
	pub const MISTYROSE: Color = Color::from(255, 228, 225, 255);
	pub const GAINSBORO: Color = Color::from(220, 220, 220, 255);
	pub const LIGHTGRAY: Color = Color::from(211, 211, 211, 255);
	pub const SILVER: Color = Color::from(192, 192, 192, 255);
	pub const DARKGRAY: Color = Color::from(169, 169, 169, 255);
	pub const GRAY: Color = Color::from(128, 128, 128, 255);
	pub const DIMGRAY: Color = Color::from(105, 105, 105, 255);
	pub const LIGHTSLATEGRAY: Color = Color::from(119, 136, 153, 255);
	pub const SLATEGRAY: Color = Color::from(112, 128, 144, 255);
	pub const DARKSLATEGRAY: Color = Color::from(47, 79, 79, 255);
	pub const BLACK: Color = Color::from(0, 0, 0, 255);
}

/*
impl Color {
	pub const INDIANRED: Color = Color::from(0.8039216, 0.36078432, 0.36078432, 1.0);
	pub const LIGHTCORAL: Color = Color::from(0.9411765, 0.5019608, 0.5019608, 1.0);
	pub const SALMON: Color = Color::from(0.98039216, 0.5019608, 0.44705883, 1.0);
	pub const DARKSALMON: Color = Color::from(0.9137255, 0.5882353, 0.47843137, 1.0);
	pub const LIGHTSALMON: Color = Color::from(1.0, 0.627451, 0.47843137, 1.0);
	pub const CRIMSON: Color = Color::from(0.8627451, 0.078431375, 0.23529412, 1.0);
	pub const RED: Color = Color::from(1.0, 0.0, 0.0, 1.0);
	pub const FIREBRICK: Color = Color::from(0.69803923, 0.13333334, 0.13333334, 1.0);
	pub const DARKRED: Color = Color::from(0.54509807, 0.0, 0.0, 1.0);
	pub const PINK: Color = Color::from(1.0, 0.7529412, 0.79607844, 1.0);
	pub const LIGHTPINK: Color = Color::from(1.0, 0.7137255, 0.75686276, 1.0);
	pub const HOTPINK: Color = Color::from(1.0, 0.4117647, 0.7058824, 1.0);
	pub const DEEPPINK: Color = Color::from(1.0, 0.078431375, 0.5764706, 1.0);
	pub const MEDIUMVIOLETRED: Color = Color::from(0.78039217, 0.08235294, 0.52156866, 1.0);
	pub const PALEVIOLETRED: Color = Color::from(0.85882354, 0.4392157, 0.5764706, 1.0);
	pub const CORAL: Color = Color::from(1.0, 0.49803922, 0.3137255, 1.0);
	pub const TOMATO: Color = Color::from(1.0, 0.3882353, 0.2784314, 1.0);
	pub const ORANGERED: Color = Color::from(1.0, 0.27058825, 0.0, 1.0);
	pub const DARKORANGE: Color = Color::from(1.0, 0.54901963, 0.0, 1.0);
	pub const ORANGE: Color = Color::from(1.0, 0.64705884, 0.0, 1.0);
	pub const GOLD: Color = Color::from(1.0, 0.84313726, 0.0, 1.0);
	pub const YELLOW: Color = Color::from(1.0, 1.0, 0.0, 1.0);
	pub const LIGHTYELLOW: Color = Color::from(1.0, 1.0, 0.8784314, 1.0);
	pub const LEMONCHIFFON: Color = Color::from(1.0, 0.98039216, 0.8039216, 1.0);
	pub const LIGHTGOLDENRODYELLOW: Color = Color::from(0.98039216, 0.98039216, 0.8235294, 1.0);
	pub const PAPAYAWHIP: Color = Color::from(1.0, 0.9372549, 0.8352941, 1.0);
	pub const MOCCASIN: Color = Color::from(1.0, 0.89411765, 0.70980394, 1.0);
	pub const PEACHPUFF: Color = Color::from(1.0, 0.85490197, 0.7254902, 1.0);
	pub const PALEGOLDENROD: Color = Color::from(0.93333334, 0.9098039, 0.6666667, 1.0);
	pub const KHAKI: Color = Color::from(0.9411765, 0.9019608, 0.54901963, 1.0);
	pub const DARKKHAKI: Color = Color::from(0.7411765, 0.7176471, 0.41960785, 1.0);
	pub const LAVENDER: Color = Color::from(0.9019608, 0.9019608, 0.98039216, 1.0);
	pub const THISTLE: Color = Color::from(0.84705883, 0.7490196, 0.84705883, 1.0);
	pub const PLUM: Color = Color::from(0.8666667, 0.627451, 0.8666667, 1.0);
	pub const VIOLET: Color = Color::from(0.93333334, 0.50980395, 0.93333334, 1.0);
	pub const ORCHID: Color = Color::from(0.85490197, 0.4392157, 0.8392157, 1.0);
	pub const FUCHSIA: Color = Color::from(1.0, 0.0, 1.0, 1.0);
	pub const MAGENTA: Color = Color::from(1.0, 0.0, 1.0, 1.0);
	pub const MEDIUMORCHID: Color = Color::from(0.7294118, 0.33333334, 0.827451, 1.0);
	pub const MEDIUMPURPLE: Color = Color::from(0.5764706, 0.4392157, 0.85882354, 1.0);
	pub const REBECCAPURPLE: Color = Color::from(0.4, 0.2, 0.6, 1.0);
	pub const BLUEVIOLET: Color = Color::from(0.5411765, 0.16862746, 0.8862745, 1.0);
	pub const DARKVIOLET: Color = Color::from(0.5803922, 0.0, 0.827451, 1.0);
	pub const DARKORCHID: Color = Color::from(0.6, 0.19607843, 0.8, 1.0);
	pub const DARKMAGENTA: Color = Color::from(0.54509807, 0.0, 0.54509807, 1.0);
	pub const PURPLE: Color = Color::from(0.5019608, 0.0, 0.5019608, 1.0);
	pub const INDIGO: Color = Color::from(0.29411766, 0.0, 0.50980395, 1.0);
	pub const SLATEBLUE: Color = Color::from(0.41568628, 0.3529412, 0.8039216, 1.0);
	pub const DARKSLATEBLUE: Color = Color::from(0.28235295, 0.23921569, 0.54509807, 1.0);
	pub const MEDIUMSLATEBLUE: Color = Color::from(0.48235294, 0.40784314, 0.93333334, 1.0);
	pub const GREENYELLOW: Color = Color::from(0.6784314, 1.0, 0.18431373, 1.0);
	pub const CHARTREUSE: Color = Color::from(0.49803922, 1.0, 0.0, 1.0);
	pub const LAWNGREEN: Color = Color::from(0.4862745, 0.9882353, 0.0, 1.0);
	pub const LIME: Color = Color::from(0.0, 1.0, 0.0, 1.0);
	pub const LIMEGREEN: Color = Color::from(0.19607843, 0.8039216, 0.19607843, 1.0);
	pub const PALEGREEN: Color = Color::from(0.59607846, 0.9843137, 0.59607846, 1.0);
	pub const LIGHTGREEN: Color = Color::from(0.5647059, 0.93333334, 0.5647059, 1.0);
	pub const MEDIUMSPRINGGREEN: Color = Color::from(0.0, 0.98039216, 0.6039216, 1.0);
	pub const SPRINGGREEN: Color = Color::from(0.0, 1.0, 0.49803922, 1.0);
	pub const MEDIUMSEAGREEN: Color = Color::from(0.23529412, 0.7019608, 0.44313726, 1.0);
	pub const SEAGREEN: Color = Color::from(0.18039216, 0.54509807, 0.34117648, 1.0);
	pub const FORESTGREEN: Color = Color::from(0.13333334, 0.54509807, 0.13333334, 1.0);
	pub const GREEN: Color = Color::from(0.0, 0.5019608, 0.0, 1.0);
	pub const DARKGREEN: Color = Color::from(0.0, 0.39215687, 0.0, 1.0);
	pub const YELLOWGREEN: Color = Color::from(0.6039216, 0.8039216, 0.19607843, 1.0);
	pub const OLIVEDRAB: Color = Color::from(0.41960785, 0.5568628, 0.13725491, 1.0);
	pub const OLIVE: Color = Color::from(0.5019608, 0.5019608, 0.0, 1.0);
	pub const DARKOLIVEGREEN: Color = Color::from(0.33333334, 0.41960785, 0.18431373, 1.0);
	pub const MEDIUMAQUAMARINE: Color = Color::from(0.4, 0.8039216, 0.6666667, 1.0);
	pub const DARKSEAGREEN: Color = Color::from(0.56078434, 0.7372549, 0.54509807, 1.0);
	pub const LIGHTSEAGREEN: Color = Color::from(0.1254902, 0.69803923, 0.6666667, 1.0);
	pub const DARKCYAN: Color = Color::from(0.0, 0.54509807, 0.54509807, 1.0);
	pub const TEAL: Color = Color::from(0.0, 0.5019608, 0.5019608, 1.0);
	pub const AQUA: Color = Color::from(0.0, 1.0, 1.0, 1.0);
	pub const CYAN: Color = Color::from(0.0, 1.0, 1.0, 1.0);
	pub const LIGHTCYAN: Color = Color::from(0.8784314, 1.0, 1.0, 1.0);
	pub const PALETURQUOISE: Color = Color::from(0.6862745, 0.93333334, 0.93333334, 1.0);
	pub const AQUAMARINE: Color = Color::from(0.49803922, 1.0, 0.83137256, 1.0);
	pub const TURQUOISE: Color = Color::from(0.2509804, 0.8784314, 0.8156863, 1.0);
	pub const MEDIUMTURQUOISE: Color = Color::from(0.28235295, 0.81960785, 0.8, 1.0);
	pub const DARKTURQUOISE: Color = Color::from(0.0, 0.80784315, 0.81960785, 1.0);
	pub const CADETBLUE: Color = Color::from(0.37254903, 0.61960787, 0.627451, 1.0);
	pub const STEELBLUE: Color = Color::from(0.27450982, 0.50980395, 0.7058824, 1.0);
	pub const LIGHTSTEELBLUE: Color = Color::from(0.6901961, 0.76862746, 0.87058824, 1.0);
	pub const POWDERBLUE: Color = Color::from(0.6901961, 0.8784314, 0.9019608, 1.0);
	pub const LIGHTBLUE: Color = Color::from(0.6784314, 0.84705883, 0.9019608, 1.0);
	pub const SKYBLUE: Color = Color::from(0.5294118, 0.80784315, 0.92156863, 1.0);
	pub const LIGHTSKYBLUE: Color = Color::from(0.5294118, 0.80784315, 0.98039216, 1.0);
	pub const DEEPSKYBLUE: Color = Color::from(0.0, 0.7490196, 1.0, 1.0);
	pub const DODGERBLUE: Color = Color::from(0.11764706, 0.5647059, 1.0, 1.0);
	pub const CORNFLOWERBLUE: Color = Color::from(0.39215687, 0.58431375, 0.92941177, 1.0);
	pub const ROYALBLUE: Color = Color::from(0.25490198, 0.4117647, 0.88235295, 1.0);
	pub const BLUE: Color = Color::from(0.0, 0.0, 1.0, 1.0);
	pub const MEDIUMBLUE: Color = Color::from(0.0, 0.0, 0.8039216, 1.0);
	pub const DARKBLUE: Color = Color::from(0.0, 0.0, 0.54509807, 1.0);
	pub const NAVY: Color = Color::from(0.0, 0.0, 0.5019608, 1.0);
	pub const MIDNIGHTBLUE: Color = Color::from(0.09803922, 0.09803922, 0.4392157, 1.0);
	pub const CORNSILK: Color = Color::from(1.0, 0.972549, 0.8627451, 1.0);
	pub const BLANCHEDALMOND: Color = Color::from(1.0, 0.92156863, 0.8039216, 1.0);
	pub const BISQUE: Color = Color::from(1.0, 0.89411765, 0.76862746, 1.0);
	pub const NAVAJOWHITE: Color = Color::from(1.0, 0.87058824, 0.6784314, 1.0);
	pub const WHEAT: Color = Color::from(0.9607843, 0.87058824, 0.7019608, 1.0);
	pub const BURLYWOOD: Color = Color::from(0.87058824, 0.72156864, 0.5294118, 1.0);
	pub const TAN: Color = Color::from(0.8235294, 0.7058824, 0.54901963, 1.0);
	pub const ROSYBROWN: Color = Color::from(0.7372549, 0.56078434, 0.56078434, 1.0);
	pub const SANDYBROWN: Color = Color::from(0.95686275, 0.6431373, 0.3764706, 1.0);
	pub const GOLDENROD: Color = Color::from(0.85490197, 0.64705884, 0.1254902, 1.0);
	pub const DARKGOLDENROD: Color = Color::from(0.72156864, 0.5254902, 0.043137256, 1.0);
	pub const PERU: Color = Color::from(0.8039216, 0.52156866, 0.24705882, 1.0);
	pub const CHOCOLATE: Color = Color::from(0.8235294, 0.4117647, 0.11764706, 1.0);
	pub const SADDLEBROWN: Color = Color::from(0.54509807, 0.27058825, 0.07450981, 1.0);
	pub const SIENNA: Color = Color::from(0.627451, 0.32156864, 0.1764706, 1.0);
	pub const BROWN: Color = Color::from(0.64705884, 0.16470589, 0.16470589, 1.0);
	pub const MAROON: Color = Color::from(0.5019608, 0.0, 0.0, 1.0);
	pub const WHITE: Color = Color::from(1.0, 1.0, 1.0, 1.0);
	pub const SNOW: Color = Color::from(1.0, 0.98039216, 0.98039216, 1.0);
	pub const HONEYDEW: Color = Color::from(0.9411765, 1.0, 0.9411765, 1.0);
	pub const MINTCREAM: Color = Color::from(0.9607843, 1.0, 0.98039216, 1.0);
	pub const AZURE: Color = Color::from(0.9411765, 1.0, 1.0, 1.0);
	pub const ALICEBLUE: Color = Color::from(0.9411765, 0.972549, 1.0, 1.0);
	pub const GHOSTWHITE: Color = Color::from(0.972549, 0.972549, 1.0, 1.0);
	pub const WHITESMOKE: Color = Color::from(0.9607843, 0.9607843, 0.9607843, 1.0);
	pub const SEASHELL: Color = Color::from(1.0, 0.9607843, 0.93333334, 1.0);
	pub const BEIGE: Color = Color::from(0.9607843, 0.9607843, 0.8627451, 1.0);
	pub const OLDLACE: Color = Color::from(0.99215686, 0.9607843, 0.9019608, 1.0);
	pub const FLORALWHITE: Color = Color::from(1.0, 0.98039216, 0.9411765, 1.0);
	pub const IVORY: Color = Color::from(1.0, 1.0, 0.9411765, 1.0);
	pub const ANTIQUEWHITE: Color = Color::from(0.98039216, 0.92156863, 0.84313726, 1.0);
	pub const LINEN: Color = Color::from(0.98039216, 0.9411765, 0.9019608, 1.0);
	pub const LAVENDERBLUSH: Color = Color::from(1.0, 0.9411765, 0.9607843, 1.0);
	pub const MISTYROSE: Color = Color::from(1.0, 0.89411765, 0.88235295, 1.0);
	pub const GAINSBORO: Color = Color::from(0.8627451, 0.8627451, 0.8627451, 1.0);
	pub const LIGHTGRAY: Color = Color::from(0.827451, 0.827451, 0.827451, 1.0);
	pub const SILVER: Color = Color::from(0.7529412, 0.7529412, 0.7529412, 1.0);
	pub const DARKGRAY: Color = Color::from(0.6627451, 0.6627451, 0.6627451, 1.0);
	pub const GRAY: Color = Color::from(0.5019608, 0.5019608, 0.5019608, 1.0);
	pub const DIMGRAY: Color = Color::from(0.4117647, 0.4117647, 0.4117647, 1.0);
	pub const LIGHTSLATEGRAY: Color = Color::from(0.46666667, 0.53333336, 0.6, 1.0);
	pub const SLATEGRAY: Color = Color::from(0.4392157, 0.5019608, 0.5647059, 1.0);
	pub const DARKSLATEGRAY: Color = Color::from(0.18431373, 0.30980393, 0.30980393, 1.0);
	pub const BLACK: Color = Color::from(0.0, 0.0, 0.0, 1.0);
}
*/

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn color_new() {
		let color = Color::new();
        assert_eq!(color, Color { r: 255.0, g: 255.0, b: 255.0, a: 255.0 });
	}
	
	#[test]
	fn color_from() {
		let color = Color::from(28, 25, 39, 255);
		assert_eq!(color, Color { r: 28.0, g: 25.0, b: 39.0, a: 255.0 });
	}

	#[test]
	fn color_from_unit_interval() {
		let color = Color::from_unit_interval(1.0, 0.0, 1.0, 1.0);
		assert_eq!(color, Color { r: 255.0, g: 0.0, b: 255.0, a: 255.0 });
	}

	#[test]
	fn color_set() {
		let mut color = Color::new();
		color.set(28.0, 25.0, 39.0, 255.0);
		assert_eq!(color, Color { r: 28.0, g: 25.0, b: 39.0, a: 255.0 });
	}

	#[test]
	fn color_unit_interval() {
		let color = Color::from(255, 0, 255, 255);
		let unit_interval = color.unit_interval();
		assert_eq!(unit_interval, (1.0, 0.0, 1.0, 1.0));
	}

	#[test]
	fn color_rgb() {
		let color = Color::from(255, 0, 255, 255);
		let rgb = color.rgb();
		assert_eq!(rgb, (255.0, 0.0, 255.0));
	}

	#[test]
	fn color_rgba() {
		let color = Color::from(255, 0, 255, 255);
		let rgb = color.rgba();
		assert_eq!(rgb, (255.0, 0.0, 255.0, 255.0));
	}

	#[test]
	fn color_add() {
		let color1 = Color::from(255, 0, 0, 255);
		let color2 = Color::from(0, 255, 0, 255);
		let color3 = color1 + color2;
		assert_eq!(color3, Color { r: 255.0, g: 255.0, b: 0.0, a: 255.0 });
	}

	#[test]
	fn color_sub() {
		let color1 = Color::from(255, 0, 255, 255);
		let color2 = Color::from(255, 0, 0, 0);
		let color3 = color1 - color2;
		assert_eq!(color3, Color { r: 0.0, g: 0.0, b: 255.0, a: 255.0 });
	}

	#[test]
	fn color_mul() {
		let color1 = Color::from(50, 0, 100, 255);
		let color2 = Color::from(3, 0, 2, 1);
		let color3 = color1 * color2;
		assert_eq!(color3, Color { r: 150.0, g: 0.0, b: 200.0, a: 255.0 });
	}

	#[test]
	fn color_div() {
		let color1 = Color::from(100, 0, 200, 255);
		let color2 = Color::from(2, 1, 4, 1);
		let color3 = color1 / color2;
		assert_eq!(color3, Color { r: 50.0, g: 0.0, b: 50.0, a: 255.0 });
	}

	#[test]
	fn color_mul_num() {
		let color = Color::from(50, 0, 100, 100);
		let color2: Color = color * 2.0;
		assert_eq!(color2, Color { r: 100.0, g: 0.0, b: 200.0, a: 200.0 });
	}

	#[test]
	fn color_div_num() {
		let color = Color::from(100, 0, 200, 200);
		let color2: Color = color / 2.0;
		assert_eq!(color2, Color { r: 50.0, g: 0.0, b: 100.0, a: 100.0 });
	}

	#[test]
	fn color_add_assign() {
		let mut color1 = Color::from(255, 0, 0, 255);
		let color2 = Color::from(0, 255, 0, 255);
		color1 += color2;
		assert_eq!(color1, Color { r: 255.0, g: 255.0, b: 0.0, a: 255.0 });
	}

	#[test]
	fn color_sub_assign() {
		let mut color1 = Color::from(255, 0, 255, 255);
		let color2 = Color::from(255, 0, 0, 0);
		color1 -= color2;
		assert_eq!(color1, Color { r: 0.0, g: 0.0, b: 255.0, a: 255.0 });
	}

	#[test]
	fn color_mul_assign() {
		let mut color1 = Color::from(50, 0, 100, 255);
		let color2 = Color::from(3, 0, 2, 1);
		color1 *= color2;
		assert_eq!(color1, Color { r: 150.0, g: 0.0, b: 200.0, a: 255.0 });
	}

	#[test]
	fn color_div_assign() {
		let mut color1 = Color::from(100, 0, 200, 255);
		let color2 = Color::from(2, 1, 4, 1);
		color1 /= color2;
		assert_eq!(color1, Color { r: 50.0, g: 0.0, b: 50.0, a: 255.0 });
	}
}