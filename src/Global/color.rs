use std::cmp::{Eq, PartialEq};
use std::convert::Into;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

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

    pub fn from<T: Into<f64>>(r: T, g: T, b: T, a: T) -> Color {
        let (r1, g1, b1, a1) = (r.into(), g.into(), b.into(), a.into());
        assert!(r1 >= 0.0 && g1 >= 0.0 && b1 >= 0.0 && a1 >= 0.0);
		assert!(r1 <= 255.0 && g1 <= 255.0 && b1 <= 255.0 && a1 <= 255.0);
        Color { r: r1 as f32, g: g1 as f32, b: b1 as f32, a: a1 as f32 }
    }

	pub fn from_unit_interval(r: f32, g: f32, b: f32, a: f32) -> Self {
		assert!(r >= 0.0 && g >= 0.0 && b >= 0.0 && a >= 0.0);
		assert!(r <= 1.0 && g <= 1.0 && b <= 1.0 && a <= 1.0);
        Color { r: r * 255.0, g: g * 255.0, b: b * 255.0, a: a * 255.0 }
	}

	pub fn unit_interval(self) -> [f32; 4] {
		[self.r / 255.0, self.g / 255.0, self.b / 255.0, self.a / 255.0]
	}

	pub const fn rgba(self) -> [f32; 4] {
		[self.r, self.g, self.b, self.a]
	}

	pub fn set(&mut self, r: f32, g: f32, b: f32, a: f32) {
		self.r = r.clamp(0.0, 255.0);
		self.g = g.clamp(0.0, 255.0);
		self.b = b.clamp(0.0, 255.0);
		self.a = a.clamp(0.0, 255.0);
	}

	pub fn add_num(&mut self, r: f32, g: f32, b: f32, a: f32) {
		self.r = (self.r + r).clamp(0.0, 255.0);
		self.g = (self.g + g).clamp(0.0, 255.0);
		self.b = (self.b + b).clamp(0.0, 255.0);
		self.a = (self.a + a).clamp(0.0, 255.0);
	}

	pub fn sub_num(&mut self, r: f32, g: f32, b: f32, a: f32) {
		self.r = (self.r - r).clamp(0.0, 255.0);
		self.g = (self.g - g).clamp(0.0, 255.0);
		self.b = (self.b - b).clamp(0.0, 255.0);
		self.a = (self.a - a).clamp(0.0, 255.0);
	}

	pub fn mul_num(&mut self, r: f32, g: f32, b: f32, a: f32) {
		self.r = (self.r * r).clamp(0.0, 255.0);
		self.g = (self.g * g).clamp(0.0, 255.0);
		self.b = (self.b * b).clamp(0.0, 255.0);
		self.a = (self.a * a).clamp(0.0, 255.0);
	}

	pub fn div_num(&mut self, r: f32, g: f32, b: f32, a: f32) {
		self.r = (self.r / r).clamp(0.0, 255.0);
		self.g = (self.g / g).clamp(0.0, 255.0);
		self.b = (self.b / b).clamp(0.0, 255.0);
		self.a = (self.a / a).clamp(0.0, 255.0);
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
			r: (self.r + other.r).clamp(0.0, 255.0),
			g: (self.g + other.g).clamp(0.0, 255.0),
			b: (self.b + other.b).clamp(0.0, 255.0),
			a: (self.a + other.a).clamp(0.0, 255.0),
		}
	}
}

impl AddAssign for Color {
	fn add_assign(&mut self, other: Color) {
		self.r = (self.r + other.r).clamp(0.0, 255.0);
		self.g = (self.g + other.g).clamp(0.0, 255.0);
		self.b = (self.b + other.b).clamp(0.0, 255.0);
		self.a = (self.a + other.a).clamp(0.0, 255.0);
	}
}

impl Sub for Color {
	type Output = Color;

	fn sub(self, other: Color) -> Color {
		Color {
			r: (self.r - other.r).clamp(0.0, 255.0),
			g: (self.g - other.g).clamp(0.0, 255.0),
			b: (self.b - other.b).clamp(0.0, 255.0),
			a: (self.a - other.a).clamp(0.0, 255.0),
		}
	}
}

impl SubAssign for Color {
	fn sub_assign(&mut self, other: Color) {
		self.r = (self.r - other.r).clamp(0.0, 255.0);
		self.g = (self.g - other.g).clamp(0.0, 255.0);
		self.b = (self.b - other.b).clamp(0.0, 255.0);
		self.a = (self.a - other.a).clamp(0.0, 255.0);
	}
}

impl Mul for Color {
	type Output = Color;

	fn mul(self, other: Color) -> Color {
		Color {
			r: (self.r * other.r).clamp(0.0, 255.0),
			g: (self.g * other.g).clamp(0.0, 255.0),
			b: (self.b * other.b).clamp(0.0, 255.0),
			a: (self.a * other.a).clamp(0.0, 255.0),
		}
	}
}

impl MulAssign for Color {
	fn mul_assign(&mut self, other: Color) {
		self.r = (self.r * other.r).clamp(0.0, 255.0);
		self.g = (self.g * other.g).clamp(0.0, 255.0);
		self.b = (self.b * other.b).clamp(0.0, 255.0);
		self.a = (self.a * other.a).clamp(0.0, 255.0);
	}
}

impl <T> Mul<T> for Color
	where T: Into<f32> {
	type Output = Color;

	fn mul(self, value: T) -> Color {
		let scale = value.into();

		Color {
			r: (self.r * scale).clamp(0.0, 255.0),
			g: (self.g * scale).clamp(0.0, 255.0),
			b: (self.b * scale).clamp(0.0, 255.0),
			a: (self.a * scale).clamp(0.0, 255.0),
		}
	}
}

impl <T> MulAssign<T> for Color
	where T: Into<f32> {
	fn mul_assign(&mut self, value: T) {
		let scale = value.into();

		self.r = (self.r * scale).clamp(0.0, 255.0);
		self.g = (self.g * scale).clamp(0.0, 255.0);
		self.b = (self.b * scale).clamp(0.0, 255.0);
		self.a = (self.a * scale).clamp(0.0, 255.0);
	}
}

impl Div for Color {
	type Output = Color;

	fn div(self, other: Color) -> Color {
		Color {
			r: (self.r / other.r).clamp(0.0, 255.0),
			g: (self.g / other.g).clamp(0.0, 255.0),
			b: (self.b / other.b).clamp(0.0, 255.0),
			a: (self.a / other.a).clamp(0.0, 255.0),
		}
	}
}

impl DivAssign for Color {
	fn div_assign(&mut self, other: Color) {
		self.r = (self.r / other.r).clamp(0.0, 255.0);
		self.g = (self.g / other.g).clamp(0.0, 255.0);
		self.b = (self.b / other.b).clamp(0.0, 255.0);
		self.a = (self.a / other.a).clamp(0.0, 255.0);
	}
}

impl <T> Div<T> for Color
	where T: Into<f32> {
	type Output = Color;

	fn div(self, value: T) -> Color {
		let scale = value.into();

		Color {
			r: (self.r / scale).clamp(0.0, 255.0),
			g: (self.g / scale).clamp(0.0, 255.0),
			b: (self.b / scale).clamp(0.0, 255.0),
			a: (self.a / scale).clamp(0.0, 255.0),
		}
	}
}

impl <T> DivAssign<T> for Color
	where T: Into<f32> {
	fn div_assign(&mut self, value: T) {
		let scale = value.into();

		self.r = (self.r / scale).clamp(0.0, 255.0);
		self.g = (self.g / scale).clamp(0.0, 255.0);
		self.b = (self.b / scale).clamp(0.0, 255.0);
		self.a = (self.a / scale).clamp(0.0, 255.0);
	}
}

impl Color {
    pub const INDIANRED: Color = Color { r: 205.0, g: 92.0, b: 92.0, a: 255.0 };
    pub const LIGHTCORAL: Color = Color { r: 240.0, g: 128.0, b: 128.0, a: 255.0 };
    pub const SALMON: Color = Color { r: 250.0, g: 128.0, b: 114.0, a: 255.0 };
    pub const DARKSALMON: Color = Color { r: 233.0, g: 150.0, b: 122.0, a: 255.0 };
    pub const LIGHTSALMON: Color = Color { r: 255.0, g: 160.0, b: 122.0, a: 255.0 };
    pub const CRIMSON: Color = Color { r: 220.0, g: 20.0, b: 60.0, a: 255.0 };
    pub const RED: Color = Color { r: 255.0, g: 0.0, b: 0.0, a: 255.0 };
    pub const FIREBRICK: Color = Color { r: 178.0, g: 34.0, b: 34.0, a: 255.0 };
    pub const DARKRED: Color = Color { r: 139.0, g: 0.0, b: 0.0, a: 255.0 };
    pub const PINK: Color = Color { r: 255.0, g: 192.0, b: 203.0, a: 255.0 };
    pub const LIGHTPINK: Color = Color { r: 255.0, g: 182.0, b: 193.0, a: 255.0 };
    pub const HOTPINK: Color = Color { r: 255.0, g: 105.0, b: 180.0, a: 255.0 };
    pub const DEEPPINK: Color = Color { r: 255.0, g: 20.0, b: 147.0, a: 255.0 };
    pub const MEDIUMVIOLETRED: Color = Color { r: 199.0, g: 21.0, b: 133.0, a: 255.0 };
    pub const PALEVIOLETRED: Color = Color { r: 219.0, g: 112.0, b: 147.0, a: 255.0 };
    pub const CORAL: Color = Color { r: 255.0, g: 127.0, b: 80.0, a: 255.0 };
    pub const TOMATO: Color = Color { r: 255.0, g: 99.0, b: 71.0, a: 255.0 };
    pub const ORANGERED: Color = Color { r: 255.0, g: 69.0, b: 0.0, a: 255.0 };
    pub const DARKORANGE: Color = Color { r: 255.0, g: 140.0, b: 0.0, a: 255.0 };
    pub const ORANGE: Color = Color { r: 255.0, g: 165.0, b: 0.0, a: 255.0 };
    pub const GOLD: Color = Color { r: 255.0, g: 215.0, b: 0.0, a: 255.0 };
    pub const YELLOW: Color = Color { r: 255.0, g: 255.0, b: 0.0, a: 255.0 };
    pub const LIGHTYELLOW: Color = Color { r: 255.0, g: 255.0, b: 224.0, a: 255.0 };
    pub const LEMONCHIFFON: Color = Color { r: 255.0, g: 250.0, b: 205.0, a: 255.0 };
    pub const LIGHTGOLDENRODYELLOW: Color = Color { r: 250.0, g: 250.0, b: 210.0, a: 255.0 };
    pub const PAPAYAWHIP: Color = Color { r: 255.0, g: 239.0, b: 213.0, a: 255.0 };
    pub const MOCCASIN: Color = Color { r: 255.0, g: 228.0, b: 181.0, a: 255.0 };
    pub const PEACHPUFF: Color = Color { r: 255.0, g: 218.0, b: 185.0, a: 255.0 };
    pub const PALEGOLDENROD: Color = Color { r: 238.0, g: 232.0, b: 170.0, a: 255.0 };
    pub const KHAKI: Color = Color { r: 240.0, g: 230.0, b: 140.0, a: 255.0 };
    pub const DARKKHAKI: Color = Color { r: 189.0, g: 183.0, b: 107.0, a: 255.0 };
    pub const LAVENDER: Color = Color { r: 230.0, g: 230.0, b: 250.0, a: 255.0 };
    pub const THISTLE: Color = Color { r: 216.0, g: 191.0, b: 216.0, a: 255.0 };
    pub const PLUM: Color = Color { r: 221.0, g: 160.0, b: 221.0, a: 255.0 };
    pub const VIOLET: Color = Color { r: 238.0, g: 130.0, b: 238.0, a: 255.0 };
    pub const ORCHID: Color = Color { r: 218.0, g: 112.0, b: 214.0, a: 255.0 };
    pub const FUCHSIA: Color = Color { r: 255.0, g: 0.0, b: 255.0, a: 255.0 };
    pub const MAGENTA: Color = Color { r: 255.0, g: 0.0, b: 255.0, a: 255.0 };
    pub const MEDIUMORCHID: Color = Color { r: 186.0, g: 85.0, b: 211.0, a: 255.0 };
    pub const MEDIUMPURPLE: Color = Color { r: 147.0, g: 112.0, b: 219.0, a: 255.0 };
    pub const REBECCAPURPLE: Color = Color { r: 102.0, g: 51.0, b: 153.0, a: 255.0 };
    pub const BLUEVIOLET: Color = Color { r: 138.0, g: 43.0, b: 226.0, a: 255.0 };
    pub const DARKVIOLET: Color = Color { r: 148.0, g: 0.0, b: 211.0, a: 255.0 };
    pub const DARKORCHID: Color = Color { r: 153.0, g: 50.0, b: 204.0, a: 255.0 };
    pub const DARKMAGENTA: Color = Color { r: 139.0, g: 0.0, b: 139.0, a: 255.0 };
    pub const PURPLE: Color = Color { r: 128.0, g: 0.0, b: 128.0, a: 255.0 };
    pub const INDIGO: Color = Color { r: 75.0, g: 0.0, b: 130.0, a: 255.0 };
    pub const SLATEBLUE: Color = Color { r: 106.0, g: 90.0, b: 205.0, a: 255.0 };
    pub const DARKSLATEBLUE: Color = Color { r: 72.0, g: 61.0, b: 139.0, a: 255.0 };
    pub const MEDIUMSLATEBLUE: Color = Color { r: 123.0, g: 104.0, b: 238.0, a: 255.0 };
    pub const GREENYELLOW: Color = Color { r: 173.0, g: 255.0, b: 47.0, a: 255.0 };
    pub const CHARTREUSE: Color = Color { r: 127.0, g: 255.0, b: 0.0, a: 255.0 };
    pub const LAWNGREEN: Color = Color { r: 124.0, g: 252.0, b: 0.0, a: 255.0 };
    pub const LIME: Color = Color { r: 0.0, g: 255.0, b: 0.0, a: 255.0 };
    pub const LIMEGREEN: Color = Color { r: 50.0, g: 205.0, b: 50.0, a: 255.0 };
    pub const PALEGREEN: Color = Color { r: 152.0, g: 251.0, b: 152.0, a: 255.0 };
    pub const LIGHTGREEN: Color = Color { r: 144.0, g: 238.0, b: 144.0, a: 255.0 };
    pub const MEDIUMSPRINGGREEN: Color = Color { r: 0.0, g: 250.0, b: 154.0, a: 255.0 };
    pub const SPRINGGREEN: Color = Color { r: 0.0, g: 255.0, b: 127.0, a: 255.0 };
    pub const MEDIUMSEAGREEN: Color = Color { r: 60.0, g: 179.0, b: 113.0, a: 255.0 };
    pub const SEAGREEN: Color = Color { r: 46.0, g: 139.0, b: 87.0, a: 255.0 };
    pub const FORESTGREEN: Color = Color { r: 34.0, g: 139.0, b: 34.0, a: 255.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 128.0, b: 0.0, a: 255.0 };
    pub const DARKGREEN: Color = Color { r: 0.0, g: 100.0, b: 0.0, a: 255.0 };
    pub const YELLOWGREEN: Color = Color { r: 154.0, g: 205.0, b: 50.0, a: 255.0 };
    pub const OLIVEDRAB: Color = Color { r: 107.0, g: 142.0, b: 35.0, a: 255.0 };
    pub const OLIVE: Color = Color { r: 128.0, g: 128.0, b: 0.0, a: 255.0 };
    pub const DARKOLIVEGREEN: Color = Color { r: 85.0, g: 107.0, b: 47.0, a: 255.0 };
    pub const MEDIUMAQUAMARINE: Color = Color { r: 102.0, g: 205.0, b: 170.0, a: 255.0 };
    pub const DARKSEAGREEN: Color = Color { r: 143.0, g: 188.0, b: 139.0, a: 255.0 };
    pub const LIGHTSEAGREEN: Color = Color { r: 32.0, g: 178.0, b: 170.0, a: 255.0 };
    pub const DARKCYAN: Color = Color { r: 0.0, g: 139.0, b: 139.0, a: 255.0 };
    pub const TEAL: Color = Color { r: 0.0, g: 128.0, b: 128.0, a: 255.0 };
    pub const AQUA: Color = Color { r: 0.0, g: 255.0, b: 255.0, a: 255.0 };
    pub const CYAN: Color = Color { r: 0.0, g: 255.0, b: 255.0, a: 255.0 };
    pub const LIGHTCYAN: Color = Color { r: 224.0, g: 255.0, b: 255.0, a: 255.0 };
    pub const PALETURQUOISE: Color = Color { r: 175.0, g: 238.0, b: 238.0, a: 255.0 };
    pub const AQUAMARINE: Color = Color { r: 127.0, g: 255.0, b: 212.0, a: 255.0 };
    pub const TURQUOISE: Color = Color { r: 64.0, g: 224.0, b: 208.0, a: 255.0 };
    pub const MEDIUMTURQUOISE: Color = Color { r: 72.0, g: 209.0, b: 204.0, a: 255.0 };
    pub const DARKTURQUOISE: Color = Color { r: 0.0, g: 206.0, b: 209.0, a: 255.0 };
    pub const CADETBLUE: Color = Color { r: 95.0, g: 158.0, b: 160.0, a: 255.0 };
    pub const STEELBLUE: Color = Color { r: 70.0, g: 130.0, b: 180.0, a: 255.0 };
    pub const LIGHTSTEELBLUE: Color = Color { r: 176.0, g: 196.0, b: 222.0, a: 255.0 };
    pub const POWDERBLUE: Color = Color { r: 176.0, g: 224.0, b: 230.0, a: 255.0 };
    pub const LIGHTBLUE: Color = Color { r: 173.0, g: 216.0, b: 230.0, a: 255.0 };
    pub const SKYBLUE: Color = Color { r: 135.0, g: 206.0, b: 235.0, a: 255.0 };
    pub const LIGHTSKYBLUE: Color = Color { r: 135.0, g: 206.0, b: 250.0, a: 255.0 };
    pub const DEEPSKYBLUE: Color = Color { r: 0.0, g: 191.0, b: 255.0, a: 255.0 };
    pub const DODGERBLUE: Color = Color { r: 30.0, g: 144.0, b: 255.0, a: 255.0 };
    pub const CORNFLOWERBLUE: Color = Color { r: 100.0, g: 149.0, b: 237.0, a: 255.0 };
    pub const ROYALBLUE: Color = Color { r: 65.0, g: 105.0, b: 225.0, a: 255.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 255.0, a: 255.0 };
    pub const MEDIUMBLUE: Color = Color { r: 0.0, g: 0.0, b: 205.0, a: 255.0 };
    pub const DARKBLUE: Color = Color { r: 0.0, g: 0.0, b: 139.0, a: 255.0 };
    pub const NAVY: Color = Color { r: 0.0, g: 0.0, b: 128.0, a: 255.0 };
    pub const MIDNIGHTBLUE: Color = Color { r: 25.0, g: 25.0, b: 112.0, a: 255.0 };
    pub const CORNSILK: Color = Color { r: 255.0, g: 248.0, b: 220.0, a: 255.0 };
    pub const BLANCHEDALMOND: Color = Color { r: 255.0, g: 235.0, b: 205.0, a: 255.0 };
    pub const BISQUE: Color = Color { r: 255.0, g: 228.0, b: 196.0, a: 255.0 };
    pub const NAVAJOWHITE: Color = Color { r: 255.0, g: 222.0, b: 173.0, a: 255.0 };
    pub const WHEAT: Color = Color { r: 245.0, g: 222.0, b: 179.0, a: 255.0 };
    pub const BURLYWOOD: Color = Color { r: 222.0, g: 184.0, b: 135.0, a: 255.0 };
    pub const TAN: Color = Color { r: 210.0, g: 180.0, b: 140.0, a: 255.0 };
    pub const ROSYBROWN: Color = Color { r: 188.0, g: 143.0, b: 143.0, a: 255.0 };
    pub const SANDYBROWN: Color = Color { r: 244.0, g: 164.0, b: 96.0, a: 255.0 };
    pub const GOLDENROD: Color = Color { r: 218.0, g: 165.0, b: 32.0, a: 255.0 };
    pub const DARKGOLDENROD: Color = Color { r: 184.0, g: 134.0, b: 11.0, a: 255.0 };
    pub const PERU: Color = Color { r: 205.0, g: 133.0, b: 63.0, a: 255.0 };
    pub const CHOCOLATE: Color = Color { r: 210.0, g: 105.0, b: 30.0, a: 255.0 };
    pub const SADDLEBROWN: Color = Color { r: 139.0, g: 69.0, b: 19.0, a: 255.0 };
    pub const SIENNA: Color = Color { r: 160.0, g: 82.0, b: 45.0, a: 255.0 };
    pub const BROWN: Color = Color { r: 165.0, g: 42.0, b: 42.0, a: 255.0 };
    pub const MAROON: Color = Color { r: 128.0, g: 0.0, b: 0.0, a: 255.0 };
    pub const WHITE: Color = Color { r: 255.0, g: 255.0, b: 255.0, a: 255.0 };
    pub const SNOW: Color = Color { r: 255.0, g: 250.0, b: 250.0, a: 255.0 };
    pub const HONEYDEW: Color = Color { r: 240.0, g: 255.0, b: 240.0, a: 255.0 };
    pub const MINTCREAM: Color = Color { r: 245.0, g: 255.0, b: 250.0, a: 255.0 };
    pub const AZURE: Color = Color { r: 240.0, g: 255.0, b: 255.0, a: 255.0 };
    pub const ALICEBLUE: Color = Color { r: 240.0, g: 248.0, b: 255.0, a: 255.0 };
    pub const GHOSTWHITE: Color = Color { r: 248.0, g: 248.0, b: 255.0, a: 255.0 };
    pub const WHITESMOKE: Color = Color { r: 245.0, g: 245.0, b: 245.0, a: 255.0 };
    pub const SEASHELL: Color = Color { r: 255.0, g: 245.0, b: 238.0, a: 255.0 };
    pub const BEIGE: Color = Color { r: 245.0, g: 245.0, b: 220.0, a: 255.0 };
    pub const OLDLACE: Color = Color { r: 253.0, g: 245.0, b: 230.0, a: 255.0 };
    pub const FLORALWHITE: Color = Color { r: 255.0, g: 250.0, b: 240.0, a: 255.0 };
    pub const IVORY: Color = Color { r: 255.0, g: 255.0, b: 240.0, a: 255.0 };
    pub const ANTIQUEWHITE: Color = Color { r: 250.0, g: 235.0, b: 215.0, a: 255.0 };
    pub const LINEN: Color = Color { r: 250.0, g: 240.0, b: 230.0, a: 255.0 };
    pub const LAVENDERBLUSH: Color = Color { r: 255.0, g: 240.0, b: 245.0, a: 255.0 };
    pub const MISTYROSE: Color = Color { r: 255.0, g: 228.0, b: 225.0, a: 255.0 };
    pub const GAINSBORO: Color = Color { r: 220.0, g: 220.0, b: 220.0, a: 255.0 };
    pub const LIGHTGRAY: Color = Color { r: 211.0, g: 211.0, b: 211.0, a: 255.0 };
    pub const SILVER: Color = Color { r: 192.0, g: 192.0, b: 192.0, a: 255.0 };
    pub const DARKGRAY: Color = Color { r: 169.0, g: 169.0, b: 169.0, a: 255.0 };
    pub const GRAY: Color = Color { r: 128.0, g: 128.0, b: 128.0, a: 255.0 };
    pub const DIMGRAY: Color = Color { r: 105.0, g: 105.0, b: 105.0, a: 255.0 };
    pub const LIGHTSLATEGRAY: Color = Color { r: 119.0, g: 136.0, b: 153.0, a: 255.0 };
    pub const SLATEGRAY: Color = Color { r: 112.0, g: 128.0, b: 144.0, a: 255.0 };
    pub const DARKSLATEGRAY: Color = Color { r: 47.0, g: 79.0, b: 79.0, a: 255.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 255.0 };
}

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
		let color_def = Color::from(28, 25, 39, 127);
		let color_fdef = Color::from(28.0, 25.0, 39.0, 127.0);
		let color_u8 = Color::from(28u8, 25u8, 39u8, 127u8);
        let color_u16 = Color::from(28u16, 25u16, 39u16, 127u16);
        let color_u32 = Color::from(28u32, 25u32, 39u32, 127u32);
        let color_i8 = Color::from(28i8, 25i8, 39i8, 127i8);
        let color_i16 = Color::from(28i16, 25i16, 39i16, 127i16);
        let color_i32 = Color::from(28i32, 25i32, 39i32, 127i32);
        let color_f32 = Color::from(28.0f32, 25.0f32, 39.0f32, 127.0f32);
        let color_f64 = Color::from(28.0f64, 25.0f64, 39.0f64, 127.0f64);
        let color = Color { r: 28.0, g: 25.0, b: 39.0, a: 127.0 };
		assert_eq!(color_def, color);
		assert_eq!(color_fdef, color);
        assert_eq!(color_u8, color);
		assert_eq!(color_u16, color);
		assert_eq!(color_u32, color);
		assert_eq!(color_i8, color);
		assert_eq!(color_i16, color);
		assert_eq!(color_i32, color);
		assert_eq!(color_f32, color);
		assert_eq!(color_f64, color);
	}

	#[test]
	fn color_from_unit_interval() {
		let color = Color::from_unit_interval(1.0, 0.0, 1.0, 1.0);
		assert_eq!(color, Color { r: 255.0, g: 0.0, b: 255.0, a: 255.0 });
	}

	#[test]
	fn color_unit_interval() {
		let color = Color::from(255, 0, 255, 255);
		let unit_interval = color.unit_interval();
		assert_eq!(unit_interval, [1.0, 0.0, 1.0, 1.0]);
	}

	#[test]
	fn color_rgba() {
		let color = Color::from(255, 0, 255, 255);
		let rgb = color.rgba();
		assert_eq!(rgb, [255.0, 0.0, 255.0, 255.0]);
	}

	#[test]
	fn color_set() {
		let mut color = Color::new();
		color.set(28.0, 25.0, 39.0, 255.0);
		assert_eq!(color, Color { r: 28.0, g: 25.0, b: 39.0, a: 255.0 });
	}

	#[test]
	fn color_self_add() {
		let mut color = Color::from(90, 100, 110, 120);
		color.add_num(10.0, 10.0, 10.0, 10.0);
		assert_eq!(color, Color { r: 100.0, g: 110.0, b: 120.0, a: 130.0 });
	}

	#[test]
	fn color_self_sub() {
		let mut color = Color::from(90, 100, 110, 120);
		color.sub_num(10.0, 10.0, 10.0, 10.0);
		assert_eq!(color, Color { r: 80.0, g: 90.0, b: 100.0, a: 110.0 });
	}

	#[test]
	fn color_self_mul() {
		let mut color = Color::from(50, 100, 30, 60);
		color.mul_num(2.0, 1.0, 3.0, 2.0);
		assert_eq!(color, Color { r: 100.0, g: 100.0, b: 90.0, a: 120.0 });
	}

	#[test]
	fn color_self_div() {
		let mut color = Color::from(50, 100, 30, 60);
		color.div_num(2.0, 1.0, 3.0, 2.0);
		assert_eq!(color, Color { r: 25.0, g: 100.0, b: 10.0, a: 30.0 });
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

	#[test]
	fn color_mul_num_assign() {
		let mut color = Color::from(50, 0, 100, 100);
		color *= 2.0;
		assert_eq!(color, Color { r: 100.0, g: 0.0, b: 200.0, a: 200.0 });
	}

	#[test]
	fn color_div_num_assign() {
		let mut color = Color::from(100, 0, 200, 200);
		color /= 2.0;
		assert_eq!(color, Color { r: 50.0, g: 0.0, b: 100.0, a: 100.0 });
	}
}