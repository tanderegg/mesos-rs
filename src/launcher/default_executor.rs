extern crate mesos;
extern crate clap;
extern crate url;

use std::env;
use std::rc::Rc;

use clap::{Arg, App};
use url::{Url};

use mesos::process;
use mesos::process::Process;
use mesos::process::pid::PID;

enum State {
    CONNECTED,
    DISCONNECTED
}

enum ContentType {
    PROTOBUF,
    JSON
}

struct DefaultExecutor {
    pid: PID,
    state: State,
    content_type: ContentType,
    launched: bool,
    shutting_down: bool,
    unhealthy: bool,
    framework_info: Option<mesos::FrameworkInfo>,
    executor_container_id: Option<u64>,
    framework_id: mesos::FrameworkId,
    executor_id: mesos::ExecutorId,
    agent_url: Url,
    sandbox_dir: String,
    launcher_dir: String,
    authorization_header: Option<String>
}

impl DefaultExecutor {
    pub fn new(framework_id: mesos::FrameworkId, executor_id: mesos::ExecutorId,
        agent_url: Url, sandbox_dir: String, launcher_dir: String, authorization_header: Option<String>) -> DefaultExecutor
    {
        DefaultExecutor{
            pid: PID::new(),
            state: State::DISCONNECTED,
            content_type: ContentType::PROTOBUF,
            launched: false,
            shutting_down: false,
            unhealthy: false,
            framework_info: None,
            executor_container_id: None,
            framework_id: framework_id,
            executor_id: executor_id,
            agent_url: agent_url,
            sandbox_dir: sandbox_dir,
            launcher_dir: launcher_dir,
            authorization_header: authorization_header
        }
    }
}

impl Process for DefaultExecutor {
    fn managed(_: bool) {}
    fn get_pid(&self) -> PID {
        return self.pid.clone();
    }
}

impl Drop for DefaultExecutor {
    fn drop(&mut self) {
        println!("Default executor {} ended.", self.get_pid())
    }
}

fn main() {
    let flags = App::new("Mesos Default Executor")
                .version("0.1")
                .arg(Arg::with_name("launcher_dir")
                        .short("l")
                        .long("launcher_dir")
                        .value_name("") // TODO: Set Default value based on environment
                        .help("Directory path of Mesos binaries."))
                .get_matches();

    // TODO: Initialize logging

    let framework_id =
        match env::var("MESOS_FRAMEWORK_ID") {
            Ok(id) => mesos::FrameworkId{ value: id },
            Err(_) => {
                println!("Expecting 'MESOS_FRAMEWORK_ID' to be set in the environment");
                ::std::process::exit(1)
            }
        };

    let executor_id =
        match env::var("MESOS_EXECUTOR_ID") {
            Ok(id) => mesos::ExecutorId{ value: id },
            Err(_) => {
                println!("Expecting 'MESOS_EXECUTOR_ID' to be set in the environment");
                ::std::process::exit(1)
            }
        };

    // TODO: Check for SSL
    let scheme = "http";

    // TODO: This actually appears to be a pointer address of the Mesos Agent PID,
    // will be tricky to implement in rust.  For now, pretend it is just an id #.
    let mesos_agent_pid =
        match env::var("MESOS_SLAVE_PID") {
            Ok(pid) => pid,
            Err(_) => {
                match env::var("MESOS_AGENT_PID") {
                    Ok(pid) => pid,
                    Err(_) => {
                        println!("Expecting 'MESOS_SLAVE_PID' or 'MESOS_AGENT_PID' to be set in the environment");
                        ::std::process::exit(1)
                    }
                }
            }
        };

    let agent_url = match Url::parse(
        &format!("{scheme}://127.0.0.1:5051/{mesos_agent_pid}/api/v1",
            scheme=scheme,
            mesos_agent_pid=mesos_agent_pid)
    ) {
        Ok(url) => url,
        Err(_ ) => {
            println!("Unable to parse agent URL");
            ::std::process::exit(1)
        }
    };

    let sandbox_dir =
        match env::var("MESOS_SANDBOX") {
            Ok(sandbox) => sandbox,
            Err(_) => {
                println!("Expecting 'MESOS_SANDBOX' to be set in the environment");
                ::std::process::exit(1)
            }
        };

    let authorization_header =
        match env::var("MESOS_EXECUTOR_AUTHENTICATION_TOKEN") {
            Ok(auth_header) => Some(format!("Bearer {auth_header}", auth_header=auth_header)),
            Err(_) => None
        };

    // Executor scope, calls destructor of `executor` before process::finalize
    {
        let executor = Rc::new(DefaultExecutor::new(
            framework_id,
            executor_id,
            agent_url,
            sandbox_dir,
            flags.value_of("launcher_dir").unwrap_or(".").to_string(),
            authorization_header
        ));
        println!("Created executor");

        let _ = process::spawn(executor.clone(), true);
        //process::wait(executor.clone());
    }

    process::finalize(true);
    ::std::process::exit(0);
}
