#![allow(dead_code)]

use objc2::ffi::NSUInteger;

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Delete,
    Backspace,
    Enter,
    Tab,
    Other(String),
}

impl ToString for Key {
    fn to_string(&self) -> String {
        match self {
            Key::A => String::from("a"),
            Key::B => String::from("b"),
            Key::C => String::from("c"),
            Key::D => String::from("d"),
            Key::E => String::from("e"),
            Key::F => String::from("f"),
            Key::G => String::from("g"),
            Key::H => String::from("h"),
            Key::I => String::from("i"),
            Key::J => String::from("j"),
            Key::K => String::from("k"),
            Key::L => String::from("l"),
            Key::M => String::from("m"),
            Key::N => String::from("n"),
            Key::O => String::from("o"),
            Key::P => String::from("p"),
            Key::Q => String::from("q"),
            Key::R => String::from("r"),
            Key::S => String::from("s"),
            Key::T => String::from("t"),
            Key::U => String::from("u"),
            Key::V => String::from("v"),
            Key::W => String::from("w"),
            Key::X => String::from("x"),
            Key::Y => String::from("y"),
            Key::Z => String::from("z"),
            Key::Num1 => String::from("1"),
            Key::Num2 => String::from("2"),
            Key::Num3 => String::from("3"),
            Key::Num4 => String::from("4"),
            Key::Num5 => String::from("5"),
            Key::Num6 => String::from("6"),
            Key::Num7 => String::from("7"),
            Key::Num8 => String::from("8"),
            Key::Num9 => String::from("9"),
            Key::Num0 => String::from("0"),
            Key::F1 => String::from_utf16(&[NSF1FUNCTIONKEY]).unwrap(),
            Key::F2 => String::from_utf16(&[NSF2FUNCTIONKEY]).unwrap(),
            Key::F3 => String::from_utf16(&[NSF3FUNCTIONKEY]).unwrap(),
            Key::F4 => String::from_utf16(&[NSF4FUNCTIONKEY]).unwrap(),
            Key::F5 => String::from_utf16(&[NSF5FUNCTIONKEY]).unwrap(),
            Key::F6 => String::from_utf16(&[NSF6FUNCTIONKEY]).unwrap(),
            Key::F7 => String::from_utf16(&[NSF7FUNCTIONKEY]).unwrap(),
            Key::F8 => String::from_utf16(&[NSF8FUNCTIONKEY]).unwrap(),
            Key::F9 => String::from_utf16(&[NSF9FUNCTIONKEY]).unwrap(),
            Key::F10 => String::from_utf16(&[NSF10FUNCTIONKEY]).unwrap(),
            Key::F11 => String::from_utf16(&[NSF11FUNCTIONKEY]).unwrap(),
            Key::F12 => String::from_utf16(&[NSF12FUNCTIONKEY]).unwrap(),
            Key::Delete => String::from("\x7F"),
            Key::Backspace => String::from("\x08"),
            Key::Enter => String::from("\r"),
            Key::Tab => String::from("\t"),
            Key::Other(key) => key.clone(),
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Shortcut {
    pub(crate) key: Key,

    capslock: bool,
    shift: bool,
    control: bool,
    option: bool,
    command: bool,
}

impl Shortcut {
    pub fn new(key: Key) -> Self {
        Self {
            key,
            capslock: false,
            shift: false,
            control: false,
            option: false,
            command: false,
        }
    }
    pub fn capslock(self, capslock: bool) -> Self {
        Self { capslock, ..self }
    }
    pub fn shift(self, shift: bool) -> Self {
        Self { shift, ..self }
    }
    pub fn control(self, control: bool) -> Self {
        Self { control, ..self }
    }
    pub fn option(self, option: bool) -> Self {
        Self { option, ..self }
    }
    pub fn command(self, command: bool) -> Self {
        Self { command, ..self }
    }

    pub(crate) fn mask(&self) -> NSUInteger {
        let mut mask = 0;

        if self.capslock {
            mask |= 1 << 16;
        }
        if self.shift {
            mask |= 1 << 17;
        }
        if self.control {
            mask |= 1 << 18;
        }
        if self.option {
            mask |= 1 << 19;
        }
        if self.command {
            mask |= 1 << 20;
        }

        mask
    }
}

// ----------------------------------------------------------------------------

const NSUPARROWFUNCTIONKEY: u16 = 0xF700;
const NSDOWNARROWFUNCTIONKEY: u16 = 0xF701;
const NSLEFTARROWFUNCTIONKEY: u16 = 0xF702;
const NSRIGHTARROWFUNCTIONKEY: u16 = 0xF703;
const NSF1FUNCTIONKEY: u16 = 0xF704;
const NSF2FUNCTIONKEY: u16 = 0xF705;
const NSF3FUNCTIONKEY: u16 = 0xF706;
const NSF4FUNCTIONKEY: u16 = 0xF707;
const NSF5FUNCTIONKEY: u16 = 0xF708;
const NSF6FUNCTIONKEY: u16 = 0xF709;
const NSF7FUNCTIONKEY: u16 = 0xF70A;
const NSF8FUNCTIONKEY: u16 = 0xF70B;
const NSF9FUNCTIONKEY: u16 = 0xF70C;
const NSF10FUNCTIONKEY: u16 = 0xF70D;
const NSF11FUNCTIONKEY: u16 = 0xF70E;
const NSF12FUNCTIONKEY: u16 = 0xF70F;
const NSF13FUNCTIONKEY: u16 = 0xF710;
const NSF14FUNCTIONKEY: u16 = 0xF711;
const NSF15FUNCTIONKEY: u16 = 0xF712;
const NSF16FUNCTIONKEY: u16 = 0xF713;
const NSF17FUNCTIONKEY: u16 = 0xF714;
const NSF18FUNCTIONKEY: u16 = 0xF715;
const NSF19FUNCTIONKEY: u16 = 0xF716;
const NSF20FUNCTIONKEY: u16 = 0xF717;
const NSF21FUNCTIONKEY: u16 = 0xF718;
const NSF22FUNCTIONKEY: u16 = 0xF719;
const NSF23FUNCTIONKEY: u16 = 0xF71A;
const NSF24FUNCTIONKEY: u16 = 0xF71B;
const NSF25FUNCTIONKEY: u16 = 0xF71C;
const NSF26FUNCTIONKEY: u16 = 0xF71D;
const NSF27FUNCTIONKEY: u16 = 0xF71E;
const NSF28FUNCTIONKEY: u16 = 0xF71F;
const NSF29FUNCTIONKEY: u16 = 0xF720;
const NSF30FUNCTIONKEY: u16 = 0xF721;
const NSF31FUNCTIONKEY: u16 = 0xF722;
const NSF32FUNCTIONKEY: u16 = 0xF723;
const NSF33FUNCTIONKEY: u16 = 0xF724;
const NSF34FUNCTIONKEY: u16 = 0xF725;
const NSF35FUNCTIONKEY: u16 = 0xF726;
const NSINSERTFUNCTIONKEY: u16 = 0xF727;
const NSDELETEFUNCTIONKEY: u16 = 0xF728;
const NSHOMEFUNCTIONKEY: u16 = 0xF729;
const NSBEGINFUNCTIONKEY: u16 = 0xF72A;
const NSENDFUNCTIONKEY: u16 = 0xF72B;
const NSPAGEUPFUNCTIONKEY: u16 = 0xF72C;
const NSPAGEDOWNFUNCTIONKEY: u16 = 0xF72D;
const NSPRINTSCREENFUNCTIONKEY: u16 = 0xF72E;
const NSSCROLLLOCKFUNCTIONKEY: u16 = 0xF72F;
const NSPAUSEFUNCTIONKEY: u16 = 0xF730;
const NSSYSREQFUNCTIONKEY: u16 = 0xF731;
const NSBREAKFUNCTIONKEY: u16 = 0xF732;
const NSRESETFUNCTIONKEY: u16 = 0xF733;
const NSSTOPFUNCTIONKEY: u16 = 0xF734;
const NSMENUFUNCTIONKEY: u16 = 0xF735;
const NSUSERFUNCTIONKEY: u16 = 0xF736;
const NSSYSTEMFUNCTIONKEY: u16 = 0xF737;
const NSPRINTFUNCTIONKEY: u16 = 0xF738;
const NSCLEARLINEFUNCTIONKEY: u16 = 0xF739;
const NSCLEARDISPLAYFUNCTIONKEY: u16 = 0xF73A;
const NSINSERTLINEFUNCTIONKEY: u16 = 0xF73B;
const NSDELETELINEFUNCTIONKEY: u16 = 0xF73C;
const NSINSERTCHARFUNCTIONKEY: u16 = 0xF73D;
const NSDELETECHARFUNCTIONKEY: u16 = 0xF73E;
const NSPREVFUNCTIONKEY: u16 = 0xF73F;
const NSNEXTFUNCTIONKEY: u16 = 0xF740;
const NSSELECTFUNCTIONKEY: u16 = 0xF741;
const NSEXECUTEFUNCTIONKEY: u16 = 0xF742;
const NSUNDOFUNCTIONKEY: u16 = 0xF743;
const NSREDOFUNCTIONKEY: u16 = 0xF744;
const NSFINDFUNCTIONKEY: u16 = 0xF745;
const NSHELPFUNCTIONKEY: u16 = 0xF746;
const NSMODESWITCHFUNCTIONKEY: u16 = 0xF74;
