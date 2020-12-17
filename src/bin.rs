use Engine::{self, Window, Size2D, Color};

fn main() {
	//let window = Engine::Window::new();

	//let shape0 = Engine::Visual::Triangle::new();

	let mut win = Window::from("TestGame", Size2D::from(856.0, 482.0), Color::BLACK);
	
	Engine::Run(win, Initialize, LoadContent, Update, Render);
}

fn Initialize() {

}

fn LoadContent() {

}

fn Update() {

}

fn Render() {

}