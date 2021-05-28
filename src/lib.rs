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
pub enum KeyCode {}

/// Queries system-wide key state.
///
/// # Examples
///
/// ```
/// // asking if left mouse button is pressed right now
/// let answer = is_key_pressed(KeyCode::MLeftButton);
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
    unsafe {}
}
