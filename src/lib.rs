use num_derive::ToPrimitive;
use num_traits::cast::ToPrimitive;

#[cfg(target_os = "windows")]
#[derive(ToPrimitive)]
pub enum KeyCode {
    MLeftButton = 0x01,
    MRightButton = 0x02,
    MMiddleButton = 0x04,
    MX1Button = 0x05,
    MX2Button = 0x06,
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
#[derive(ToPrimitive)]
// see /usr/include/X11/keysymdef.h
pub enum KeyCode {
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
