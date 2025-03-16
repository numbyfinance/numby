#[derive(Debug)]
pub struct StaticFile {
    pub file_name: &'static str,
    pub name: &'static str,
    pub mime: &'static str,
}

/// From "/home/justin/dev/github.com/numbyfinance/numby/web/static/numby.png"
#[allow(non_upper_case_globals)]
pub static numby_png: StaticFile = StaticFile {
    file_name: "/home/justin/dev/github.com/numbyfinance/numby/web/static/numby.png",
    name: "/static/numby-23d7cea197ef59fc3f03a02be4a5340c.png",
    mime: "image/png",
};

/// From "/home/justin/dev/github.com/numbyfinance/numby/web/static/kafka.webp"
#[allow(non_upper_case_globals)]
pub static kafka_webp: StaticFile = StaticFile {
    file_name: "/home/justin/dev/github.com/numbyfinance/numby/web/static/kafka.webp",
    name: "/static/kafka-8ba8f0d085e7b79106daf6b4a6d0cf24.webp",
    mime: "image/webp",
};

/// From "/home/justin/dev/github.com/numbyfinance/numby/web/static/herta.webp"
#[allow(non_upper_case_globals)]
pub static herta_webp: StaticFile = StaticFile {
    file_name: "/home/justin/dev/github.com/numbyfinance/numby/web/static/herta.webp",
    name: "/static/herta-44cf247c84e1b91f0ad7c82524c779fd.webp",
    mime: "image/webp",
};

/// From "/home/justin/dev/github.com/numbyfinance/numby/web/static/tailwind.css"
#[allow(non_upper_case_globals)]
pub static tailwind_css: StaticFile = StaticFile {
    file_name: "/home/justin/dev/github.com/numbyfinance/numby/web/static/tailwind.css",
    name: "/static/tailwind-25f9c097e8a6ff32f834f595af6bc7f1.css",
    mime: "text/css",
};

/// From "/home/justin/dev/github.com/numbyfinance/numby/web/static/base.js"
#[allow(non_upper_case_globals)]
pub static base_js: StaticFile = StaticFile {
    file_name: "/home/justin/dev/github.com/numbyfinance/numby/web/static/base.js",
    name: "/static/base-e06d39e6fba9351ee3088a8c1d185cd2.js",
    mime: "application/javascript",
};

pub mod vendor {
    use super::StaticFile;

    /// From "/home/justin/dev/github.com/numbyfinance/numby/web/static/vendor/datastar.js"
    #[allow(non_upper_case_globals)]
    pub static datastar_js: StaticFile = StaticFile {
        file_name: "/home/justin/dev/github.com/numbyfinance/numby/web/static/vendor/datastar.js",
        name: "/static/vendor/datastar-fd8d8391bf8c78fb8621e489ed8655c0.js",
        mime: "application/javascript",
    };
}

#[allow(dead_code)]
impl StaticFile {
    /// Get a single `StaticFile` by name, if it exists.
    #[must_use]
    pub fn get(name: &str) -> Option<&'static Self> {
        if let Some(pos) = STATICS.iter().position(|&s| name == s.name) {
            Some(STATICS[pos])
        } else {
            None
        }
    }
}

impl std::fmt::Display for StaticFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

static STATICS: &[&StaticFile] = &[
    &numby_png,
    &kafka_webp,
    &herta_webp,
    &tailwind_css,
    &base_js,
    &vendor::datastar_js
];
