#![no_std]
#![no_main]

use wups::*;
use wut::prelude::*;
use wut::*;

WUPS_PLUGIN_NAME!("Plugin Template");

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
