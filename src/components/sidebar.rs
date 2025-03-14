use maud::{Markup, html};

pub fn component() -> Markup {
    html! {
        div class="min-w-fit" {
            div class="fixed inset-0 bg-gray-900/30 z-40 lg:hidden lg:z-auto transition-opacity duration-200" aria-hidden="true" x-cloak {}

            div class="flex justify-between mb-10 pr-3 sm:px-2" {
                a class="block" href="index.html" {
                    svg class="fill-violet-500" xmlns="http://www.w3.org/2000/svg" width="32" height="32" {
                        path d="M31.956 14.8C31.372 6.92 25.08.628 17.2.044V5.76a9.04 9.04 0 0 0 9.04 9.04h5.716ZM14.8 26.24v5.716C6.92 31.372.63 25.08.044 17.2H5.76a9.04 9.04 0 0 1 9.04 9.04Zm11.44-9.04h5.716c-.584 7.88-6.876 14.172-14.756 14.756V26.24a9.04 9.04 0 0 1 9.04-9.04ZM.044 14.8C.63 6.92 6.92.628 14.8.044V5.76a9.04 9.04 0 0 1-9.04 9.04H.044Z";
                    }
                }
            }

            div class="space-y-8" {
                div {
                    h3 class="text-xs uppercase text-gray-400 dark:text-gray-500 font-semibold pl-3" {
                        span class="hidden lg:block lg:sidebar-expanded:hidden 2xl:hidden text-center w-6"
                             aria-hidden="true" { "•••" }
                        span class="lg:hidden lg:sidebar-expanded:block 2xl:block" { "Pages" }
                    }
                    ul class="mt-3" {
                        // Dashboard (example of a complete list item)
                        li class="pl-4 pr-3 py-2 rounded-lg mb-0.5 last:mb-0" x-data="{ open: false }" {
                            a class="block text-gray-800 dark:text-gray-100 hover:text-gray-900 dark:hover:text-white truncate transition"
                              href="#0"
                               {
                                div class="flex items-center justify-between" {
                                    div class="flex items-center" {
                                        svg class="shrink-0 fill-current text-gray-400 dark:text-gray-500"
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="16"
                                            height="16"
                                            viewBox="0 0 16 16" {
                                            path d="M5.936.278A7.983 7.983 0 0 1 8 0a8 8 0 1 1-8 8c0-.722.104-1.413.278-2.064a1 1 0 1 1 1.932.516A5.99 5.99 0 0 0 2 8a6 6 0 1 0 6-6c-.53 0-1.045.076-1.548.21A1 1 0 1 1 5.936.278Z";
                                            path d="M6.068 7.482A2.003 2.003 0 0 0 8 10a2 2 0 1 0-.518-3.932L3.707 2.293a1 1 0 0 0-1.414 1.414l3.775 3.775Z";
                                        }
                                        span class="text-sm font-medium ml-4 lg:opacity-0 lg:sidebar-expanded:opacity-100 2xl:opacity-100 duration-200" { "Dashboard" }
                                    }
                                    // Icon
                                    div class="flex shrink-0 ml-2 lg:opacity-0 lg:sidebar-expanded:opacity-100 2xl:opacity-100 duration-200" {
                                        svg class="w-3 h-3 shrink-0 ml-1 fill-current text-gray-400 dark:text-gray-500"
                                            viewBox="0 0 12 12" {
                                            path d="M5.9 11.4L.5 6l1.4-1.4 4 4 4-4L11.3 6z";
                                        }
                                    }
                                }
                            }
                            div class="lg:hidden lg:sidebar-expanded:block 2xl:block" {
                                ul class="pl-8 mt-1" {
                                    li class="mb-1 last:mb-0" {
                                        a class="block text-gray-500/90 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition truncate"
                                          href="index.html" {
                                            span class="text-sm font-medium lg:opacity-0 lg:sidebar-expanded:opacity-100 2xl:opacity-100 duration-200" { "Main" }
                                        }
                                    }
                                    li class="mb-1 last:mb-0" {
                                        a class="block text-gray-500/90 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition truncate"
                                          href="analytics.html" {
                                            span class="text-sm font-medium lg:opacity-0 lg:sidebar-expanded:opacity-100 2xl:opacity-100 duration-200" { "Analytics" }
                                        }
                                    }
                                    li class="mb-1 last:mb-0" {
                                        a class="block text-gray-500/90 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition truncate"
                                          href="fintech.html" {
                                            span class="text-sm font-medium lg:opacity-0 lg:sidebar-expanded:opacity-100 2xl:opacity-100 duration-200" { "Fintech" }
                                        }
                                    }
                                }
                            }
                        }

                        // For the remaining list items, we'll just add placeholders
                        // In a real implementation, you would repeat the pattern above for each item
                        // E-Commerce
                        li class="pl-4 pr-3 py-2 rounded-lg mb-0.5 last:mb-0" x-data="{ open: false }" {
                            // (similar structure as Dashboard item)
                            // Note: Only including the Dashboard item in full as requested
                            a class="block text-gray-800 dark:text-gray-100 hover:text-gray-900 dark:hover:text-white truncate transition"
                              href="#0" {
                                "E-Commerce (full implementation would follow Dashboard pattern)"
                            }
                        }

                        // Additional menu items would follow...
                    }
                }

                // More group
                div {
                    h3 class="text-xs uppercase text-gray-400 dark:text-gray-500 font-semibold pl-3" {
                        span class="hidden lg:block lg:sidebar-expanded:hidden 2xl:hidden text-center w-6" aria-hidden="true" { "•••" }
                        span class="lg:hidden lg:sidebar-expanded:block 2xl:block" { "More" }
                    }

                    // More menu items would be implemented here...
                }
            }

            // Expand / collapse button
            div class="pt-3 hidden lg:inline-flex 2xl:hidden justify-end mt-auto" {
                div class="w-12 pl-4 pr-3 py-2" {
                    button class="text-gray-400 hover:text-gray-500 dark:text-gray-500 dark:hover:text-gray-400 transition-colors" {
                        span class="sr-only" { "Expand / collapse sidebar" }
                        svg class="shrink-0 fill-current text-gray-400 dark:text-gray-500 sidebar-expanded:rotate-180"
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            viewBox="0 0 16 16" {
                            path d="M15 16a1 1 0 0 1-1-1V1a1 1 0 1 1 2 0v14a1 1 0 0 1-1 1ZM8.586 7H1a1 1 0 1 0 0 2h7.586l-2.793 2.793a1 1 0 1 0 1.414 1.414l4.5-4.5A.997.997 0 0 0 12 8.01M11.924 7.617a.997.997 0 0 0-.217-.324l-4.5-4.5a1 1 0 0 0-1.414 1.414L8.586 7M12 7.99a.996.996 0 0 0-.076-.373Z";
                        }
                    }
                }
            }
        }
    }
}
