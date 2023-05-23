use ds::DriverStation;
use network_tables::v4;

use crate::robot::DriverStationState;

pub struct NetworkTables(pub v4::Client);

/* 
impl Default for NetworkTables {
    fn default() -> Self {
        return NetworkTables {
            0: v4::Client::try_new_w_config(
                "172.0.0.1", 
                v4::Config {
                    connect_timeout: 0,
                    disconnect_retry_interval: 0,
                    should_reconnect: should_reconnect(),
                    on_announce: 0,
                    on_un_announce: 0,
                    on_disconnect: disconnect(),
                    on_reconnect: reconnect(),
                })
            };
    }
}


#[tauri::command]
pub fn publish(nt: tauri::State<NetworkTables>) -> () {
    nt.0.publish_topic(name, topic_type, properties);
    return;
}

fn reconnect() {

}

fn should_reconnect() -> bool {
    return true
}

fn disconnect() {

}
*/