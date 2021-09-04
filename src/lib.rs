use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use num_traits::cast::ToPrimitive;

#[cfg(target_os = "windows")]
#[derive(Copy, Clone, ToPrimitive, FromPrimitive, PartialEq)]
pub enum KeyCode {
    None,
    KBackspace = 0x08,
    KTab = 0x09,
    KEnter = 0x0D,
    KShift = 0x10,
    KCtrl = 0x11,
    KAlt = 0x12,
    KCapsLock = 0x14,
    KEsc = 0x1B,
    KSpace = 0x20,
    KPageUp = 0x21,
    KPageDown = 0x22,
    KEnd = 0x23,
    KHome = 0x24,
    KArrowLeft = 0x25,
    KArrowUp = 0x26,
    KArrowRight = 0x27,
    KArrowDown = 0x28,
    KPrintScreen = 0x2C,
    KInsert = 0x2D,
    KDelete = 0x2E,
    K0 = 0x30,
    K1 = 0x31,
    K2 = 0x32,
    K3 = 0x33,
    K4 = 0x34,
    K5 = 0x35,
    K6 = 0x36,
    K7 = 0x37,
    K8 = 0x38,
    K9 = 0x39,
    KA = 0x41,
    KB = 0x42,
    KC = 0x43,
    KD = 0x44,
    KE = 0x45,
    KF = 0x46,
    KG = 0x47,
    KH = 0x48,
    KI = 0x49,
    KJ = 0x4A,
    KK = 0x4B,
    KL = 0x4C,
    KM = 0x4D,
    KN = 0x4E,
    KO = 0x4F,
    KP = 0x50,
    KQ = 0x51,
    KR = 0x52,
    KS = 0x53,
    KT = 0x54,
    KU = 0x55,
    KV = 0x56,
    KW = 0x57,
    KX = 0x58,
    KY = 0x59,
    KZ = 0x5A,
    KF1 = 0x70,
    KF2 = 0x71,
    KF3 = 0x72,
    KF4 = 0x73,
    KF5 = 0x74,
    KF6 = 0x75,
    KF7 = 0x76,
    KF8 = 0x77,
    KF9 = 0x78,
    KF10 = 0x79,
    KF11 = 0x7A,
    KF12 = 0x7B,
}

#[cfg(target_os = "linux")]
#[derive(Copy, Clone, ToPrimitive, FromPrimitive, PartialEq)]
// see /usr/include/X11/keysymdef.h
pub enum KeyCode {
    None,
    KBackspace = 0xff08,
    KTab = 0xff09,
    KEnter = 0xff0d,
    KShift = 0xffe1,
    KCtrl = 0xffe3,
    KAlt = 0xffe9,
    KCapsLock = 0xffe5,
    KEsc = 0xff1b,
    KSpace = 0xff80,
    KPageUp = 0xff9a,
    KPageDown = 0xff9b,
    KEnd = 0xff9c,
    KHome = 0xff95,
    KArrowLeft = 0x08fb,
    KArrowUp = 0x08fc,
    KArrowRight = 0x08fd,
    KArrowDown = 0x08fe,
    KPrintScreen = 0xfd1d,
    KInsert = 0xff9e,
    KDelete = 0xff9f,
    K0 = 0x0030,
    K1 = 0x0031,
    K2 = 0x0032,
    K3 = 0x0033,
    K4 = 0x0034,
    K5 = 0x0035,
    K6 = 0x0036,
    K7 = 0x0037,
    K8 = 0x0038,
    K9 = 0x0039,
    KA = 0x0041,
    KB = 0x0042,
    KC = 0x0043,
    KD = 0x0044,
    KE = 0x0045,
    KF = 0x0046,
    KG = 0x0047,
    KH = 0x0048,
    KI = 0x0049,
    KJ = 0x004A,
    KK = 0x004B,
    KL = 0x004C,
    KM = 0x004D,
    KN = 0x004E,
    KO = 0x004F,
    KP = 0x0050,
    KQ = 0x0051,
    KR = 0x0052,
    KS = 0x0053,
    KT = 0x0054,
    KU = 0x0055,
    KV = 0x0056,
    KW = 0x0057,
    KX = 0x0058,
    KY = 0x0059,
    KZ = 0x005A,
    KF1 = 0xffbe,
    KF2 = 0xffbf,
    KF3 = 0xffc0,
    KF4 = 0xffc1,
    KF5 = 0xffc2,
    KF6 = 0xffc3,
    KF7 = 0xffc4,
    KF8 = 0xffc5,
    KF9 = 0xffc6,
    KF10 = 0xffc7,
    KF11 = 0xffc8,
    KF12 = 0xffc9,
}

/// Queries system-wide key state.
///
/// # Examples
///
/// ```
/// // asking if Escape key is pressed right now
/// let answer = is_key_pressed(KeyCode::KEsc);
///
/// // asking if keyboard key T is pressed right now
/// let answer = is_key_pressed(KeyCode::KT);
/// ```
pub fn is_key_pressed(key: KeyCode) -> bool {
    #[cfg(target_os = "windows")]
    unsafe {
        let key_code = key.to_i32().unwrap();
        let res = winapi::um::winuser::GetAsyncKeyState(key_code) as i32 & 0x8000;
        if res == 0 {
            false
        } else {
            true
        }
    }

    #[cfg(target_os = "linux")]
    unsafe {
        use x11::xlib::*;

        let key_code = key.to_u64().unwrap();

        let display = XOpenDisplay(0 as *const i8);
        let mut key_return = [0i8; 32];
        XQueryKeymap(display, &mut key_return as *mut i8);
        let kc = XKeysymToKeycode(display, key_code);
        let is_pressed = key_return[(kc >> 3) as usize] & (1 << (kc & 7));
        XCloseDisplay(display);

        if is_pressed == 0 {
            false
        } else {
            true
        }
    }
}

/// Returns the keycode from the key name.
///
/// # Examples
///
/// ```
/// assert_eq!(string_to_key("T"), KeyCode::KT);
/// assert_eq!(string_to_key("some invalid text"), KeyCode::None);
/// ```
pub fn string_to_key(key_name: &str) -> KeyCode {
    match key_name {
        "0" => KeyCode::K0,
        "1" => KeyCode::K1,
        "2" => KeyCode::K2,
        "3" => KeyCode::K3,
        "4" => KeyCode::K4,
        "5" => KeyCode::K5,
        "6" => KeyCode::K6,
        "7" => KeyCode::K7,
        "8" => KeyCode::K8,
        "9" => KeyCode::K9,
        "A" => KeyCode::KA,
        "B" => KeyCode::KB,
        "C" => KeyCode::KC,
        "D" => KeyCode::KD,
        "E" => KeyCode::KE,
        "F" => KeyCode::KF,
        "G" => KeyCode::KG,
        "H" => KeyCode::KH,
        "I" => KeyCode::KI,
        "J" => KeyCode::KJ,
        "K" => KeyCode::KK,
        "L" => KeyCode::KL,
        "M" => KeyCode::KM,
        "N" => KeyCode::KN,
        "O" => KeyCode::KO,
        "P" => KeyCode::KP,
        "Q" => KeyCode::KQ,
        "R" => KeyCode::KR,
        "S" => KeyCode::KS,
        "T" => KeyCode::KT,
        "U" => KeyCode::KU,
        "V" => KeyCode::KV,
        "W" => KeyCode::KW,
        "X" => KeyCode::KX,
        "Y" => KeyCode::KY,
        "Z" => KeyCode::KZ,
        "F1" => KeyCode::KF1,
        "F2" => KeyCode::KF2,
        "F3" => KeyCode::KF3,
        "F4" => KeyCode::KF4,
        "F5" => KeyCode::KF5,
        "F6" => KeyCode::KF6,
        "F7" => KeyCode::KF7,
        "F8" => KeyCode::KF8,
        "F9" => KeyCode::KF9,
        "F10" => KeyCode::KF10,
        "F11" => KeyCode::KF11,
        "F12" => KeyCode::KF12,
        "Back Space" => KeyCode::KBackspace,
        "Tab" => KeyCode::KTab,
        "Enter" => KeyCode::KEnter,
        "Shift" => KeyCode::KShift,
        "Ctrl" => KeyCode::KCtrl,
        "Alt" => KeyCode::KAlt,
        "Caps Lock" => KeyCode::KCapsLock,
        "Esc" => KeyCode::KEsc,
        "Space" => KeyCode::KSpace,
        "Page Up" => KeyCode::KPageUp,
        "Page Down" => KeyCode::KPageDown,
        "End" => KeyCode::KEnd,
        "Home" => KeyCode::KHome,
        "Arrow Left" => KeyCode::KArrowLeft,
        "Arrow Up" => KeyCode::KArrowUp,
        "Arrow Right" => KeyCode::KArrowRight,
        "Arrow Down" => KeyCode::KArrowDown,
        "Print Screen" => KeyCode::KPrintScreen,
        "Insert" => KeyCode::KInsert,
        "Delete" => KeyCode::KDelete,
        _ => KeyCode::None,
    }
}

/// Returns key name from keycode.
///
/// # Examples
///
/// ```
/// assert_eq!("T", get_key_name(KeyCode::KT));
/// ```
pub fn get_key_name(key: KeyCode) -> String {
    match key {
        KeyCode::None => String::from(""),
        KeyCode::K0 => String::from("0"),
        KeyCode::K1 => String::from("1"),
        KeyCode::K2 => String::from("2"),
        KeyCode::K3 => String::from("3"),
        KeyCode::K4 => String::from("4"),
        KeyCode::K5 => String::from("5"),
        KeyCode::K6 => String::from("6"),
        KeyCode::K7 => String::from("7"),
        KeyCode::K8 => String::from("8"),
        KeyCode::K9 => String::from("9"),
        KeyCode::KA => String::from("A"),
        KeyCode::KB => String::from("B"),
        KeyCode::KC => String::from("C"),
        KeyCode::KD => String::from("D"),
        KeyCode::KE => String::from("E"),
        KeyCode::KF => String::from("F"),
        KeyCode::KG => String::from("G"),
        KeyCode::KH => String::from("H"),
        KeyCode::KI => String::from("I"),
        KeyCode::KJ => String::from("J"),
        KeyCode::KK => String::from("K"),
        KeyCode::KL => String::from("L"),
        KeyCode::KM => String::from("M"),
        KeyCode::KN => String::from("N"),
        KeyCode::KO => String::from("O"),
        KeyCode::KP => String::from("P"),
        KeyCode::KQ => String::from("Q"),
        KeyCode::KR => String::from("R"),
        KeyCode::KS => String::from("S"),
        KeyCode::KT => String::from("T"),
        KeyCode::KU => String::from("U"),
        KeyCode::KV => String::from("V"),
        KeyCode::KW => String::from("W"),
        KeyCode::KX => String::from("X"),
        KeyCode::KY => String::from("Y"),
        KeyCode::KZ => String::from("Z"),
        KeyCode::KF1 => String::from("F1"),
        KeyCode::KF2 => String::from("F2"),
        KeyCode::KF3 => String::from("F3"),
        KeyCode::KF4 => String::from("F4"),
        KeyCode::KF5 => String::from("F5"),
        KeyCode::KF6 => String::from("F6"),
        KeyCode::KF7 => String::from("F7"),
        KeyCode::KF8 => String::from("F8"),
        KeyCode::KF9 => String::from("F9"),
        KeyCode::KF10 => String::from("F10"),
        KeyCode::KF11 => String::from("F11"),
        KeyCode::KF12 => String::from("F12"),
        KeyCode::KBackspace => String::from("Back Space"),
        KeyCode::KTab => String::from("Tab"),
        KeyCode::KEnter => String::from("Enter"),
        KeyCode::KShift => String::from("Shift"),
        KeyCode::KCtrl => String::from("Ctrl"),
        KeyCode::KAlt => String::from("Alt"),
        KeyCode::KCapsLock => String::from("Caps Lock"),
        KeyCode::KEsc => String::from("Esc"),
        KeyCode::KSpace => String::from("Space"),
        KeyCode::KPageUp => String::from("Page Up"),
        KeyCode::KPageDown => String::from("Page Down"),
        KeyCode::KEnd => String::from("End"),
        KeyCode::KHome => String::from("Home"),
        KeyCode::KArrowLeft => String::from("Arrow Left"),
        KeyCode::KArrowUp => String::from("Arrow Up"),
        KeyCode::KArrowRight => String::from("Arrow Right"),
        KeyCode::KArrowDown => String::from("Arrow Down"),
        KeyCode::KPrintScreen => String::from("Print Screen"),
        KeyCode::KInsert => String::from("Insert"),
        KeyCode::KDelete => String::from("Delete"),
    }
}
