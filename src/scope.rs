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
}
impl Streams {
    fn new() -> Self {
        Self {
            self_in_scope: false,
            expirer: streams::Expirer::new(),
            latest: streams::Latest::new(),
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
}
