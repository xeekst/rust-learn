#[macro_use]
extern crate windows_service;

use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use std::{fs, process, thread};
use windows_service::service_dispatcher;

define_windows_service!(ffi_service_main, my_service_main);

use windows_service::service::{
    ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus, ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};

// arguments 是服务启动的参数比如: sc start service-name arg1 arg2, arguments 就是 ["arg1","arg2"]
fn my_service_main(arguments: Vec<OsString>) {
    if let Err(_e) = run_service(arguments) {
        // Handle errors in some way.
    }
}

fn run_service(arguments: Vec<OsString>) -> Result<(), windows_service::Error> {
    let (shutdown_tx, shutdown_rx) = mpsc::channel();
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop => {
                shutdown_tx.send(()).unwrap();
                // Handle stop event and return control back to the system.
                ServiceControlHandlerResult::NoError
            }
            // All services must accept Interrogate even if it's a no-op.
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    // Register system service event handler
    let status_handle =
        service_control_handler::register("xtest-service-rust22321", event_handler)?;

    let next_status = ServiceStatus {
        // Should match the one from system service registry
        service_type: ServiceType::OWN_PROCESS,
        // The new state
        current_state: ServiceState::Running,
        // Accept stop events when running
        controls_accepted: ServiceControlAccept::STOP
            | ServiceControlAccept::SHUTDOWN
            | ServiceControlAccept::TIME_CHANGE,
        // Used to report an error when starting or stopping only, otherwise must be zero
        exit_code: ServiceExitCode::Win32(0),
        // Only used for pending states, otherwise must be zero
        checkpoint: 0,
        // Only used for pending states, otherwise must be zero
        wait_hint: Duration::default(),
        process_id: None,
    };

    // Tell the system that the service is running now
    status_handle.set_service_status(next_status)?;

    thread::spawn(|| loop {
        fs::write("thread-xtest-service", "working").unwrap();
        thread::sleep(Duration::from_secs(2));
    });
    //dosth
    loop {
        if PathBuf::from("win-service-rs").exists() {
            let text = fs::read_to_string("win-service-rs").unwrap();
            fs::write(
                "win-service-rs",
                format!("{text}\nwin-service-rs:{:?}", arguments),
            )
            .unwrap();
        } else {
            fs::write("win-service-rs", format!("win-service-rs:{:?}", arguments)).unwrap();
        }

        // Poll shutdown event.
        match shutdown_rx.recv_timeout(Duration::from_secs(3)) {
            // Break the loop either upon stop or channel disconnect
            Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

            // Continue work if no events were received within the timeout
            Err(mpsc::RecvTimeoutError::Timeout) => (),
        };
    }

    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    Ok(())
}

fn main() -> Result<(), windows_service::Error> {
    // Register generated `ffi_service_main` with the system and start the service, blocking
    // this thread until the service is stopped.
    //The name of a service to be run in this service process.
    //If the service is installed with the SERVICE_WIN32_OWN_PROCESS service type, this member is ignored, but cannot be NULL. This member can be an empty string ("").
    //If the service is installed with the SERVICE_WIN32_SHARE_PROCESS service type, this member specifies the name of the service that uses the ServiceMain function pointed to by the lpServiceProc member.
    service_dispatcher::start("xtest-service-rust", ffi_service_main)?;
    Ok(())
}
