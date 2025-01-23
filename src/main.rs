#![no_std]
#![no_main]

use wups::*;
use wut::prelude::*;
use wut::*;

WUPS_PLUGIN_NAME!("Plugin Template");
WUPS_PLUGIN_DESCRIPTION!("Rust WUPS Plugin Template");
WUPS_PLUGIN_VERSION!("v0.1");
WUPS_PLUGIN_AUTHOR!("29th-day");
WUPS_PLUGIN_LICENSE!("GPL3");

#[on_initialize(Udp)]
fn init() {
    println!("plugin init");
}

#[on_application_start(Udp)]
fn start() {
    println!("application start");
}

#[on_deinitialize(Udp)]
fn deinit() {
    println!("plugin deinit");
}
