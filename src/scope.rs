use super::*;
pub mod path {
    use super::*;
    #[derive(Clone, Copy)]
    pub enum Crate {
        Streams(Streams),
        ConstantGetter,
        NoneGetter,
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
        streams: Streams,
        streams_in_scope: bool,
        constant_getter_in_scope: bool,
        none_getter_in_scope: bool,
    }
    struct Streams {
        expirer_in_scope: bool,
        latest_in_scope: bool,
    }
    impl Crate {
        pub fn string_path(&self, path: path::Crate) -> String {
            match path {
                path::Crate::Streams(streams_type) => {
                    let module_path: String = if self.streams_in_scope {
                        "streams::".into()
                    } else {
                        "rrtk::streams::".into()
                    };
                    self.streams.string_path(module_path, streams_type)
                }
                path::Crate::ConstantGetter => {
                    if self.constant_getter_in_scope {
                        "ConstantGetter".into()
                    } else {
                        "rrtk::ConstantGetter".into()
                    }
                }
                path::Crate::NoneGetter => {
                    if self.none_getter_in_scope {
                        "NoneGetter".into()
                    } else {
                        "rrtk::NoneGetter".into()
                    }
                }
            }
        }
    }
    impl Streams {
        fn string_path(&self, module_path: String, path: path::Streams) -> String {
            match path {
                path::Streams::Expirer => {
                    if self.expirer_in_scope {
                        "Expirer".into()
                    } else {
                        module_path + "Expirer".into()
                    }
                }
                path::Streams::Latest => {
                    if self.latest_in_scope {
                        "Latest".into()
                    } else {
                        module_path + "Latest".into()
                    }
                }
            }
        }
    }
}
