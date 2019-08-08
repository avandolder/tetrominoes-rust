use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use ggez::{
    event::{Axis, Button, EventHandler},
    input::{
        gamepad::GamepadId,
        keyboard::{KeyCode, KeyMods},
        mouse::MouseButton,
    },
    Context, GameResult,
};

pub type StateRef = Rc<RefCell<Box<dyn State>>>;

pub trait State {
    fn on_start(&mut self, _ctx: &mut Context, _prev_state: Option<StateRef>) {}

    fn on_stop(&mut self, _ctx: &mut Context) {}

    fn on_resume(&mut self, _ctx: &mut Context) {}

    fn on_pause(&mut self, _ctx: &mut Context) {}

    fn update(&mut self, ctx: &mut Context) -> GameResult<Transition>;

    fn draw(&mut self, ctx: &mut Context) -> GameResult;

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> Transition {
        Transition::None
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> Transition {
        Transition::None
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Transition {
        Transition::None
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) -> Transition {
        Transition::None
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) -> Transition {
        Transition::None
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
    ) -> Transition {
        Transition::None
    }

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) -> Transition {
        Transition::None
    }

    fn gamepad_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _btn: Button,
        _id: GamepadId,
    ) -> Transition {
        Transition::None
    }

    fn gamepad_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _btn: Button,
        _id: GamepadId,
    ) -> Transition {
        Transition::None
    }

    fn gamepad_axis_event(
        &mut self,
        _ctx: &mut Context,
        _axis: Axis,
        _value: f32,
        _id: GamepadId,
    ) -> Transition {
        Transition::None
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) -> Transition {
        Transition::None
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> Transition {
        Transition::None
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) -> Transition {
        Transition::None
    }
}

pub enum Transition {
    None,
    Pop,
    Push(Box<dyn State>),
    Switch(Box<dyn State>),
}

pub struct StateManager {
    states: Vec<StateRef>,
}

impl StateManager {
    pub fn new(ctx: &mut Context, mut st: Box<dyn State>) -> StateManager {
        st.on_start(ctx, None);
        StateManager { states: vec![Rc::new(RefCell::new(st))] }
    }

    fn handle_transition(&mut self, ctx: &mut Context, t: Transition) {
        let current_state = self.states.len() - 1;

        match t {
            Transition::None => {}
            Transition::Pop => {
                self.current_state().on_stop(ctx);
                self.states.pop();
                if self.states.is_empty() {
                    std::process::exit(0);
                }
                self.current_state().on_resume(ctx);
            }
            Transition::Push(mut st) => {
                self.current_state().on_pause(ctx);
                let prev = self.states[current_state].clone();
                st.on_start(ctx, Some(prev));
                self.states.push(Rc::new(RefCell::new(st)));
            }
            Transition::Switch(mut st) => {
                self.current_state().on_stop(ctx);
                let prev = self.states.last().unwrap().clone();
                st.on_start(ctx, Some(prev));
                self.states[current_state] = Rc::new(RefCell::new(st));
            }
        }
    }

    fn current_state(&mut self) -> RefMut<Box<dyn State>> {
        let strc = self.states.last_mut().unwrap();
        Rc::get_mut(strc).unwrap().borrow_mut()
    }
}

impl EventHandler for StateManager {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let t = self.current_state().update(ctx)?;
        self.handle_transition(ctx, t);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.current_state().draw(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let t = self.current_state().mouse_button_down_event(ctx, button, x, y);
        self.handle_transition(ctx, t);
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let t = self.current_state().mouse_button_up_event(ctx, button, x, y);
        self.handle_transition(ctx, t);
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
        let t = self.current_state().mouse_motion_event(ctx, x, y, dx, dy);
        self.handle_transition(ctx, t);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        let t = self.current_state().mouse_wheel_event(ctx, x, y);
        self.handle_transition(ctx, t);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
    ) {
        let t = self.current_state().key_down_event(ctx, keycode, keymods, repeat);
        self.handle_transition(ctx, t);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        let t = self.current_state().key_up_event(ctx, keycode, keymods);
        self.handle_transition(ctx, t);
    }

    fn text_input_event(&mut self, ctx: &mut Context, character: char) {
        let t = self.current_state().text_input_event(ctx, character);
        self.handle_transition(ctx, t);
    }

    fn gamepad_button_down_event(&mut self, ctx: &mut Context, btn: Button, id: GamepadId) {
        let t = self.current_state().gamepad_button_down_event(ctx, btn, id);
        self.handle_transition(ctx, t);
    }

    fn gamepad_button_up_event(&mut self, ctx: &mut Context, btn: Button, id: GamepadId) {
        let t = self.current_state().gamepad_button_up_event(ctx, btn, id);
        self.handle_transition(ctx, t);
    }

    fn gamepad_axis_event(&mut self, ctx: &mut Context, axis: Axis, value: f32, id: GamepadId) {
        let t = self.current_state().gamepad_axis_event(ctx, axis, value, id);
        self.handle_transition(ctx, t);
    }

    fn focus_event(&mut self, ctx: &mut Context, gained: bool) {
        let t = self.current_state().focus_event(ctx, gained);
        self.handle_transition(ctx, t);
    }

    fn quit_event(&mut self, ctx: &mut Context) -> bool {
        let t = self.current_state().quit_event(ctx);
        if let Transition::None = t {
            false
        } else {
            self.handle_transition(ctx, t);
            true
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let t = self.current_state().resize_event(ctx, width, height);
        self.handle_transition(ctx, t);
    }
}
