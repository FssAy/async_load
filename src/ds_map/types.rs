use super::*;

pub(super) type FnAddr = usize;
pub(super) type EventPerformAsync = unsafe extern "C" fn(c_int, c_int);
pub(super) type DsMapCreate = extern "C" fn(c_int, ...) -> c_int;
pub(super) type DsMapAddDouble = extern "C" fn(c_int, *const c_char, c_double) -> bool;
pub(super) type DsMapAddString = extern "C" fn(c_int, *const c_char, *const c_char) -> bool;

#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum EventType {
    WebImageLoad = 60,
    WebSoundLoad = 61,
    WebAsync = 62,
    DialogAsync = 63,
    WebIAP = 66,
    WebCloud = 67,
    WebNetworking = 68,
    WebSteam = 69,
    #[default]
    Social = 70,
    PushNotification = 71,
    AsyncSaveLoad = 72,
    AudioRecording = 73,
    AudioPlayback = 74,
    SystemEvent = 75,
    MessageEvent = 76,
}

impl From<GMLDouble> for EventType {
    /// Converts a numeric value into the `EventType`.
    ///
    /// If the value is invalid it will return `EventType::Social`.
    fn from(value: GMLDouble) -> Self {
        let value = value as i32;
        match value {
            60..=63 | 66..=76 => {
                unsafe { std::mem::transmute::<_, Self>(value) }
            }
            _ => {
                EventType::default()
            }
        }
    }
}
