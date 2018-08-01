//
//  logging_custom.rs
//  oslog/examples
//
//  Created by Søren Mortensen on 31/07/2018.
//  Copyright © 2018 Søren Mortensen. All rights reserved.
//

#[macro_use]
extern crate log;
extern crate oslog;

fn main() {
    oslog::init_custom("com.econobox.oslog", "examples").expect("Could not initialize oslog");

    trace!("Trace level message from oslog examples");
    debug!("Debug level message from oslog examples");
    info!("Info level message from oslog examples");
    warn!("Warn level message from oslog examples");
    error!("Error level message from oslog examples");
}
