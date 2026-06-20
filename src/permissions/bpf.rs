use std::sync::Mutex;

use pcap::{Capture, Device};

static CACHED: Mutex<Option<bool>> = Mutex::new(None);
pub struct BpfAccess;

impl BpfAccess {
    pub fn is_available() -> bool {
        let mut cached = CACHED.lock().unwrap();
        *cached.get_or_insert_with(Self::try_open)
    }

    pub fn help_message() -> &'static str {
        if Self::is_available() {
            ""
        } else {
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                "Packet capture requires permission setup.\nRun: sudo mewn --setup"
            }

            #[cfg(target_os = "windows")]
            {
                "Packet capture requires Npcap and Administrator.\nInstall: https://npcap.com  then run as Administrator"
            }

            #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
            {
                "Packet capture unavailable on this platform."
            }
        }
    }

    fn try_open() -> bool {
        let Some(device) = Device::lookup().ok().flatten() else {
            return false;
        };
        Capture::from_device(device).and_then(|capture| capture.open()).is_ok()
    }
}
