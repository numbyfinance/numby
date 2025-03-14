use axum::{http::StatusCode, response::IntoResponse};
use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::session::AuthSession;

pub struct Layout {
    pub session: AuthSession,
    pub head: Option<Markup>,
}

impl Layout {
    pub fn new(session: AuthSession) -> Self {
        Self {
            session,
            head: None,
        }
    }

    pub fn with_head(mut self, head: Markup) -> Self {
        self.head = Some(head);
        self
    }

    pub fn render(&self, title: &str, children: Markup) -> Markup {
        html! {
           (DOCTYPE)
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, height=device-height, initial-scale=1.0, minimum-scale=1.0";

            link rel="stylesheet" href="/static/tailwind.css";

            script type="module" src="/static/vendor/datastar.js" {}
            script type="module" src="/static/base.js" {}

            script {
                (PreEscaped("
                    if (localStorage.getItem('dark-mode') === 'false' || !('dark-mode' in localStorage)) {
                        document.querySelector('html').classList.remove('dark');
                        document.querySelector('html').style.colorScheme = 'light';
                    } else {
                        document.querySelector('html').classList.add('dark');
                        document.querySelector('html').style.colorScheme = 'dark';
                    }
                "))
            }

            @if let Some(head) = &self.head {
                (head)
            }

            title { (title) }

            body {
                (children)
            }
        }
    }
}
