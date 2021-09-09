use crate::{Size2D, Color};
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Debug)]
pub struct Window {
    pub title: &'static str,
    pub icon: &'static str,
    pub size: Size2D,
    pub max: Size2D,
	pub min: Size2D,
	pub aspect_ratio: (u8, u8),
    pub resizable: bool,
    pub fullscreen: bool,
    pub maximized: bool,
    pub visible: bool,
    pub focused: bool,
    pub transparent: bool,
    pub decorations: bool,
    pub always_on_top: bool,
	pub color: Color,
}

impl Window {
    pub const fn new() -> Window {
        Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        }
    }

    pub const fn from(title: &'static str, size: Size2D, color: Color) -> Window {
        Window {
            title,
            icon: "",
            size,
            max: size,
			min: Size2D { x: 0.0, y: 0.0 },
			aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
			color,
        }
    }

    pub const fn title(mut self, title: &'static str) -> Window {
        self.title = title;
        self
    }

    pub const fn icon(mut self, path: &'static str) -> Window {
        self.icon = path;
        self
    }

    pub const fn size(mut self, size: Size2D) -> Window {
        self.size = size;
        self
    }

    pub const fn max(mut self, max: Size2D) -> Window {
        self.max = max;
        self
    }

    pub const fn min(mut self, min: Size2D) -> Window {
        self.min = min;
        self
    }

    pub const fn aspect_ratio(mut self, aspect_ratio: (u8, u8)) -> Window {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub const fn resizable(mut self, resizable: bool) -> Window {
        self.resizable = resizable;
        self
    }

    pub const fn fullscreen(mut self, fullscreen: bool) -> Window {
        self.fullscreen = fullscreen;
        self
    }

    pub const fn maximized(mut self, maximized: bool) -> Window {
        self.maximized = maximized;
        self
    }

    pub const fn visible(mut self, visible: bool) -> Window {
        self.visible = visible;
        self
    }

    pub const fn focused(mut self, focused: bool) -> Window {
        self.focused = focused;
        self
    }

    pub const fn transparent(mut self, transparent: bool) -> Window {
        self.transparent = transparent;
        self
    }

    pub const fn decorations(mut self, decorations: bool) -> Window {
        self.decorations = decorations;
        self
    }

    pub const fn always_on_top(mut self, always_on_top: bool) -> Window {
        self.always_on_top = always_on_top;
        self
    }

    pub const fn color(mut self, color: Color) -> Window {
        self.color = color;
        self
    }

	//pub const fn draw() {}
}

impl Eq for Window {}

impl PartialEq for Window {
    fn eq(&self, other: &Window) -> bool {
        self.title == other.title &&
        self.icon == other.icon &&
        self.size == other.size &&
        self.max == other.max &&
        self.min == other.min &&
        self.resizable == other.resizable &&
        self.fullscreen == other.fullscreen &&
        self.maximized == other.maximized &&
        self.visible == other.visible &&
        self.focused == other.focused &&
        self.transparent == other.transparent &&
        self.decorations == other.decorations &&
        self.always_on_top == other.always_on_top &&
        self.color == other.color
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_new() {
        let window = Window::new();
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_from() {
        let window = Window::from("title", Size2D::from(100, 100), Color::BLACK);
        let raw = Window {
            title: "title",
            icon: "",
            size: Size2D { x: 100.0, y: 100.0 },
            max: Size2D { x: 100.0, y: 100.0 },
            min: Size2D { x: 0.0, y: 0.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_title() {
        let window = Window::new().title("new title");
        let raw = Window {
            title: "new title",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_icon() {
        let window = Window::new().icon("path/for/icon.png");
        let raw = Window {
            title: "window",
            icon: "path/for/icon.png",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_size() {
        let window = Window::new().size(Size2D::from(100, 100));
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 100.0, y: 100.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_max() {
        let window = Window::new().max(Size2D::from(500, 500));
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 500.0, y: 500.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_min() {
        let window = Window::new().min(Size2D::from(100, 100));
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 100.0, y: 100.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_aspect_ratio() {
        let window = Window::new().aspect_ratio((40, 1));
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (40, 1),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_resizable() {
        let window = Window::new().resizable(false);
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: false,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_fullscreen() {
        let window = Window::new().fullscreen(true);
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: true,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_maximized() {
        let window = Window::new().maximized(true);
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: true,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_visible() {
        let window = Window::new().visible(false);
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: false,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_focused() {
        let window = Window::new().focused(false);
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: false,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_transparent() {
        let window = Window::new().transparent(true);
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: true,
            decorations: true,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_decorations() {
        let window = Window::new().decorations(false);
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: false,
            always_on_top: false,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_always_on_top() {
        let window = Window::new().always_on_top(true);
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: true,
            color: Color::BLACK,
        };
        assert_eq!(window, raw);
    }

    #[test]
    fn window_color() {
        let window = Window::new().color(Color::AQUA);
        let raw = Window {
            title: "window",
            icon: "",
            size: Size2D { x: 856.0, y: 482.0 },
            max: Size2D { x: 856.0, y: 482.0 },
            min: Size2D { x: 160.0, y: 90.0 },
            aspect_ratio: (16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            color: Color::AQUA,
        };
        assert_eq!(window, raw);
    }
}