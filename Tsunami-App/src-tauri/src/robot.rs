use std::sync::Mutex;
use ds::{DriverStation, Alliance};

pub struct DriverStationState(pub Mutex<DriverStation>);

impl Default for DriverStationState {
    fn default() -> Self {
        return DriverStationState(Mutex::new(DriverStation::new("172.0.0.1", Alliance(1), 8230)));
    }
}

macro_rules! ds_tauri {
    ($func:ident) => {
        #[tauri::command]
        pub fn $func(ds: tauri::State<'_, DriverStationState>) -> () {
            let _ = ds.0.lock().unwrap().$func();
        }
    };
}

ds_tauri!(enable);
ds_tauri!(disable);
ds_tauri!(estop);
ds_tauri!(restart_code);
ds_tauri!(restart_roborio);

macro_rules! ds_tauri_getter {
    ($func:ident, $res:ty) => {
        #[tauri::command]
        pub fn $func(ds: tauri::State<'_, DriverStationState>) -> $res {
            return ds.0.lock().unwrap().$func();
        }
    };
}

ds_tauri_getter!(battery_voltage, f32);
//ds_tauri_getter!(ds_mode, DsMode); Can't Serialize/Deserialize
ds_tauri_getter!(enabled, bool);
ds_tauri_getter!(estopped, bool);
ds_tauri_getter!(team_number, u32);
//ds_tauri_getter!(udp_queue, Vec<UdpTag>); Can't Serialize/Deserialize

macro_rules! ds_tauri_setter {
    ($func:ident, $param:ty) => {
        #[tauri::command]
        pub fn $func(ds: tauri::State<'_, DriverStationState>, val: $param) -> () {
            return ds.0.lock().unwrap().$func(val);
        }
    };
}

ds_tauri_setter!(set_team_number, u32);
ds_tauri_setter!(set_use_usb, bool);