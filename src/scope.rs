#![allow(unused)]
use super::*;
pub mod path {
    use super::*;
    #[derive(Clone, Copy)]
    pub enum Crate {
        ConstantGetter,
        NoneGetter,
        Streams(Streams),
    }
    #[derive(Clone, Copy)]
    pub enum Streams {
        Expirer,
        Latest,
    }
}
pub mod scope {
    use super::*;
    pub struct Crate {
        constant_getter: ConstantGetter,
        none_getter: NoneGetter,
        streams: Streams,
    }
    struct ConstantGetter {
        self_in_scope: bool,
    }
    struct NoneGetter {
        self_in_scope: bool,
    }
    struct Streams {
        self_in_scope: bool,
        expirer_in_scope: bool,
        latest_in_scope: bool,
    }
    impl Crate {
        pub fn string_path(&self, path: path::Crate) -> String {
            let crate_path: String = "rrtk::".into();
            match path {
                path::Crate::ConstantGetter => self.constant_getter.string_path(crate_path),
                path::Crate::NoneGetter => self.none_getter.string_path(crate_path),
                path::Crate::Streams(stream_type) => self.streams.string_path(crate_path, stream_type),
            }
        }
    }
    impl ConstantGetter {
        fn string_path(&self, super_path: String) -> String {
            let name = "ConstantGetter";
            if self.self_in_scope {
                name.into()
            } else {
                super_path + name
            }
        }
    }
    impl NoneGetter {
        fn string_path(&self, super_path: String) -> String {
            let name = "NoneGetter";
            if self.self_in_scope {
                name.into()
            } else {
                super_path + name
            }
        }
    }
    impl Streams {
        fn string_path(&self, super_path: String, path: path::Streams) -> String {
            todo!();
        }
    }
}
