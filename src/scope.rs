#![allow(unused)]
use super::*;
pub struct Crate {
    constant_getter: ConstantGetter,
    none_getter: NoneGetter,
    streams: Streams,
}
impl Crate {
    pub fn new() -> Self {
        Self {
            constant_getter: ConstantGetter::new(),
            none_getter: NoneGetter::new(),
            streams: Streams::new(),
        }
    }
    pub fn string_path(&self, path: path::Crate) -> String {
        let crate_path: String = "rrtk::".into();
        match path {
            path::Crate::ConstantGetter => self.constant_getter.string_path(crate_path),
            path::Crate::NoneGetter => self.none_getter.string_path(crate_path),
            path::Crate::Streams(stream_type) => self.streams.string_path(crate_path, stream_type),
        }
    }
}
struct ConstantGetter {
    self_in_scope: bool,
}
impl ConstantGetter {
    fn new() -> Self {
        Self {
            self_in_scope: false,
        }
    }
    fn string_path(&self, super_path: String) -> String {
        let name = "ConstantGetter";
        if self.self_in_scope {
            name.into()
        } else {
            super_path + name
        }
    }
}
struct NoneGetter {
    self_in_scope: bool,
}
impl NoneGetter {
    fn new() -> Self {
        Self {
            self_in_scope: false,
        }
    }
    fn string_path(&self, super_path: String) -> String {
        let name = "NoneGetter";
        if self.self_in_scope {
            name.into()
        } else {
            super_path + name
        }
    }
}
struct Streams {
    self_in_scope: bool,
    expirer: streams::Expirer,
    latest: streams::Latest,
    control: streams::Control,
}
impl Streams {
    fn new() -> Self {
        Self {
            self_in_scope: false,
            expirer: streams::Expirer::new(),
            latest: streams::Latest::new(),
            control: streams::Control::new(),
        }
    }
    fn string_path(&self, super_path: String, path: path::Streams) -> String {
        let name = "streams::";
        let name: String = if self.self_in_scope {
            name.into()
        } else {
            super_path + name
        };
        match path {
            path::Streams::Expirer => self.expirer.string_path(name),
            path::Streams::Latest => self.latest.string_path(name),
        }
    }
}
mod streams {
    pub struct Expirer {
        self_in_scope: bool,
    }
    impl Expirer {
        pub fn new() -> Self {
            Self {
                self_in_scope: false,
            }
        }
        pub fn string_path(&self, super_path: String) -> String {
            let name = "Expirer";
            if self.self_in_scope {
                name.into()
            } else {
                super_path + name
            }
        }
    }
    pub struct Latest {
        self_in_scope: bool,
    }
    impl Latest {
        pub fn new() -> Self {
            Self {
                self_in_scope: false,
            }
        }
        pub fn string_path(&self, super_path: String) -> String {
            let name = "Latest";
            if self.self_in_scope {
                name.into()
            } else {
                super_path + name
            }
        }
    }
    pub struct Control {
        self_in_scope: bool,
        command_pid: control::CommandPID,
        ewma_stream: control::EWMAStream,
        moving_average_stream: control::MovingAverageStream,
        pid_controller_stream: control::PIDControllerStream,
    }
    impl Control {
        pub fn new() -> Self {
            Self {
                self_in_scope: false,
                command_pid: control::CommandPID::new(),
                ewma_stream: control::EWMAStream::new(),
                moving_average_stream: control::MovingAverageStream::new(),
                pid_controller_stream: control::PIDControllerStream::new(),
            }
        }
    }
    mod control {
        pub struct CommandPID {
            self_in_scope: bool,
        }
        impl CommandPID {
            pub fn new() -> Self {
                Self {
                    self_in_scope: false,
                }
            }
            pub fn string_path(&self, super_path: String) -> String {
                let name = "CommandPID";
                if self.self_in_scope {
                    name.into()
                } else {
                    super_path + name
                }
            }
        }
        pub struct EWMAStream {
            self_in_scope: bool,
        }
        impl EWMAStream {
            pub fn new() -> Self {
                Self {
                    self_in_scope: false,
                }
            }
            pub fn string_path(&self, super_path: String) -> String {
                let name = "EWMAStream";
                if self.self_in_scope {
                    name.into()
                } else {
                    super_path + name
                }
            }
        }
        pub struct MovingAverageStream {
            self_in_scope: bool,
        }
        impl MovingAverageStream {
            pub fn new() -> Self {
                Self {
                    self_in_scope: false,
                }
            }
            pub fn string_path(&self, super_path: String) -> String {
                let name = "MovingAverageStream";
                if self.self_in_scope {
                    name.into()
                } else {
                    super_path + name
                }
            }
        }
        pub struct PIDControllerStream {
            self_in_scope: bool,
        }
        impl PIDControllerStream {
            pub fn new() -> Self {
                Self {
                    self_in_scope: false,
                }
            }
            pub fn string_path(&self, super_path: String) -> String {
                let name = "PIDControllerStream";
                if self.self_in_scope {
                    name.into()
                } else {
                    super_path + name
                }
            }
        }
    }
}
