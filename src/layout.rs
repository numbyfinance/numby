use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::session::AuthSession;
use crate::r#static::statics;

#[allow(dead_code)]
pub struct Layout {
    pub session: AuthSession,
    pub head: Option<Markup>,
    pub sidebar: bool,
    pub content: bool,
}

#[allow(dead_code)]
impl Layout {
    pub fn new(session: AuthSession) -> Self {
        Self {
            session,
            head: None,
            sidebar: true,
            content: true,
        }
    }

    pub fn head(mut self, head: Markup) -> Self {
        self.head = Some(head);
        self
    }

    pub fn no_sidebar(mut self) -> Self {
        self.sidebar = false;
        self
    }

    pub fn no_content(mut self) -> Self {
        self.content = false;
        self
    }

    pub fn render(&self, title: &str, children: Markup) -> Markup {
        html! {
           (DOCTYPE)
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, height=device-height, initial-scale=1.0, minimum-scale=1.0";

            link rel="stylesheet" href=(statics::tailwind_css.name);

            script type="module" src=(statics::datastar_js.name) {}
            script type="module" src=(statics::base_js.name) {}

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

            body class="font-inter antialiased bg-gray-100 dark:bg-gray-900 text-gray-600 dark:text-gray-400" {
                @if self.sidebar {
                   (crate::components::sidebar())
                }

                @if self.content {
                    div class="relative flex flex-col flex-1 overflow-y-auto overflow-x-hidden" {
                        main class="grow" {
                            (children)
                        }
                    }
                } @else {
                    (children)
                }
            }
        }
    }
}
