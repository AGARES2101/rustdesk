// RuwareDesk: Custom default server settings

use hbb_common::config::DEFAULT_SETTINGS;

pub fn init_ruwaredesk_defaults() {
    let mut defaults = DEFAULT_SETTINGS.write().unwrap();
    
    // Set RuwareDesk server settings as defaults
    defaults.insert("custom-rendezvous-server".to_string(), "85.193.89.34".to_string());
    defaults.insert("relay-server".to_string(), "85.193.89.34".to_string());
    defaults.insert("key".to_string(), "q0mCBMBPG6hgRPCLXhaFdjWCaE4gdix8h+bZOP7wQSw=".to_string());
    
    log::info!("RuwareDesk: Default server settings initialized");
}
