#![allow(unused)]
use super::*;
fn reduce(super_path: String, name: &str, in_scope: bool) -> String {
    if in_scope {
        name.into()
    } else {
        super_path + name
    }
}
pub struct Crate {
    constant_getter: bool,
    none_getter: bool,
    streams: Streams,
}
impl Crate {
    pub fn new() -> Self {
        Self {
            constant_getter: false,
            none_getter: false,
            streams: Streams::new(),
        }
    }
    pub fn string_path(&self, path: path::Crate) -> String {
        let crate_path: String = "rrtk::".into();
        match path {
            path::Crate::ConstantGetter => {
                reduce(crate_path, "ConstantGetter", self.constant_getter)
            }
            path::Crate::NoneGetter => reduce(crate_path, "NoneGetter", self.none_getter),
            path::Crate::Streams(stream_type) => self.streams.string_path(crate_path, stream_type),
        }
    }
}
struct Streams {
    self_in_scope: bool,
    expirer: bool,
    latest: bool,
    control: streams::Control,
}
impl Streams {
    fn new() -> Self {
        Self {
            self_in_scope: false,
            expirer: false,
            latest: false,
            control: streams::Control::new(),
        }
    }
    fn string_path(&self, super_path: String, path: path::Streams) -> String {
        let name = reduce(super_path, "streams::", self.self_in_scope);
        match path {
            path::Streams::Expirer => reduce(name, "Expirer", self.expirer),
            path::Streams::Latest => reduce(name, "Latest", self.latest),
            path::Streams::Control(stream_type) => self.control.string_path(name, stream_type),
        }
    }
}
mod streams {
    use super::*;
    pub struct Control {
        self_in_scope: bool,
        command_pid: bool,
        ewma_stream: bool,
        moving_average_stream: bool,
        pid_controller_stream: bool,
    }
    impl Control {
        pub fn new() -> Self {
            Self {
                self_in_scope: false,
                command_pid: false,
                ewma_stream: false,
                moving_average_stream: false,
                pid_controller_stream: false,
            }
        }
        pub fn string_path(&self, super_path: String, path: path::streams::Control) -> String {
            let name = reduce(super_path, "control::", self.self_in_scope);
            match path {
                path::streams::Control::CommandPID => reduce(name, "CommandPID", self.command_pid),
                path::streams::Control::EWMAStream => reduce(name, "EWMAStream", self.ewma_stream),
                path::streams::Control::MovingAverageStream => {
                    reduce(name, "MovingAverageStream", self.moving_average_stream)
                }
                path::streams::Control::PIDControllerStream => {
                    reduce(name, "PIDControllerStream", self.pid_controller_stream)
                }
            }
        }
    }
}
