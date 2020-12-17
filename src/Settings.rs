use super::{Color, Size2D, glfw::{self, Context}};

pub struct Window {
    pub title: &'static str,
    pub icon: &'static str,
    pub size: Size2D,
    pub max: Size2D,
    pub min: Size2D,
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
            size: Size2D::from(856.0, 482.0),
            max: Size2D::from(856.0, 482.0),
            min: Size2D::from(160.0, 90.0),
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
            min: Size2D::from(160.0, 90.0),
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

    pub const fn icon(mut self, icon: &'static str) -> Window {
        self.icon = icon;
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

	pub const fn draw() {}
}
