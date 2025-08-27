//! Channel bindings for all buttons and axis the joystick has

/// All different rotational axis
use evdev::{EventType, InputEvent};

/// The different rotational axis
#[derive(Debug, Clone, Copy)]
pub enum Axis {
    /// Axis 1
    StickUpDown,
    /// Axis 0
    StickLeftRight,
    /// Axis 5
    StickYaw,
    /// Axis 17
    JoyPadUpDown,
    /// Axis 16
    JoyPadLeftRight,
    /// Axis 6
    Throttle,
}

impl TryFrom<u16> for Axis {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Axis::StickUpDown),
            0 => Ok(Axis::StickLeftRight),
            5 => Ok(Axis::StickYaw),
            17 => Ok(Axis::JoyPadUpDown),
            16 => Ok(Axis::JoyPadLeftRight),
            6 => Ok(Axis::Throttle),
            _ => Err(()),
        }
    }
}

/// All different buttons
#[derive(Debug, Clone, Copy)]
pub enum Button {
    /// 288
    Trigger,
    /// 289
    StickSide,
    /// 290
    Button3,
    /// 291
    Button4,
    /// 292
    Button5,
    /// 293
    Button6,
    /// 294
    Button7,
    /// 295
    Button8,
    /// 296
    Button9,
    /// 297
    Button10,
    /// 298
    Button11,
    /// 299
    Button12,
}

impl TryFrom<u16> for Button {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            288 => Ok(Button::Trigger),
            289 => Ok(Button::StickSide),
            290 => Ok(Button::Button3),
            291 => Ok(Button::Button4),
            292 => Ok(Button::Button5),
            293 => Ok(Button::Button6),
            294 => Ok(Button::Button7),
            295 => Ok(Button::Button8),
            296 => Ok(Button::Button9),
            297 => Ok(Button::Button10),
            298 => Ok(Button::Button11),
            299 => Ok(Button::Button12),
            _ => Err(()),
        }
    }
}

/// Any input event
#[derive(Debug, Clone, Copy)]
pub enum AxisEvent {
    /// Button and true if on false if off
    Button(Button, bool),
    /// Axis and angle
    Axis(Axis, i32),
}

impl TryFrom<InputEvent> for AxisEvent {
    type Error = ();
    fn try_from(event: InputEvent) -> Result<Self, Self::Error> {
        match event.event_type() {
            EventType::ABSOLUTE => {
                let axis = Axis::try_from(event.code())?;
                Ok(AxisEvent::Axis(axis, event.value()))
            }
            EventType::KEY => {
                let state = event.value() == 1;
                let b = Button::try_from(event.code())?;
                Ok(Self::Button(b, state))
            }
            _ => Err(()),
        }
    }
}
