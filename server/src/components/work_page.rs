use maud::{html, Render, Markup};
use crate::components::{ColorBlocks, Image, Layout, Nav};
use crate::library::work::Work;

pub struct WorkPage {
    work: &'static Work,
    description: &'static str,
    haiku: [&'static str; 3],
    name_slot: Markup,
}

impl WorkPage {
    pub fn new(
        work: &'static Work,
        description: &'static str,
        haiku: [&'static str; 3],
    ) -> WorkPage {
        WorkPage {
            work,
            description,
            haiku,
            name_slot: html! { (work.name) },
        }
    }

    pub fn name_slot(mut self, name_slot: Markup) -> Self {
        self.name_slot = name_slot;
        self
    }
}

impl Render for WorkPage {
    fn render(&self) -> maud::Markup {
        let html = page_html(self.work, self.haiku, &self.name_slot);
        let layout = Layout::new(
            self.work.name,
            self.description,
            html,
        );
        layout.render()
    }
}

fn page_html(work: &Work, haiku: [&'static str; 3], name_slot: &Markup) -> Markup {
    html! {
        div class="w-full md:h-lvh p-4 md:p-8 xl:p-12" {
            (Nav::new())

            // justify-end + flex-col-reverse makes the content appear at the top of the column.
            main
                class="flex flex-col-reverse justify-end md:flex-row h-[calc(100%-3rem)] md:h-[calc(100%-5rem)]"
                {
                (name_section(work, haiku, name_slot))
                div class="mobile-spacer h-8 md:hidden shrink-0" {}
                (work_section(work))
            }
        }
    }
}

fn name_section(work: &Work, haiku: [&'static str; 3], name_slot: &Markup) -> Markup {
    html! {
        div class="
            name-section
            w-full md:w-fit shrink-0
            flex flex-col
            md:pr-20
            z-10
            " {
            div class="name w-full h-full flex flex-col justify-center items-start relative" {
                div class="name-text flex flex-col" {
                    div class="name flex flex-col items-start" {
                        (ColorBlocks::new(&work.palette))
                        div class="spacer h-2 md:h-4" {}
                        h1 class="text-2xl md:text-4xl xl:text-6xl uppercase font-extralight text-neutral-700 dark:text-neutral-100 tracking-widest" {
                            (name_slot)
                        }
                    }

                    div class="spacer h-3 md:h-6" {}
                    p class="text-sm/[1.25] xl:text-xl font-normal text-neutral-600 dark:text-neutral-300 tracking-wide tabular-nums" {
                        (work.year) " · " (work.month)
                    }
                    div class="spacer h-3 md:h-6" {}

                    div class="haiku" {
                        p class="text-base/[1.25] xl:text-2xl italic lowercase font-extralight -translate-y-[0.25rem] text-neutral-600 dark:text-neutral-300 tracking-wide" {
                            (haiku[0])
                        }
                        div class="spacer w-full h-1 md:h-2" {}
                        p class="text-base/[1.25] xl:text-2xl italic lowercase font-extralight -translate-y-[0.25rem] text-neutral-600 dark:text-neutral-300 tracking-wide" {
                            (haiku[1])
                        }
                        div class="spacer w-full h-1 md:h-2" {}
                        p class="text-base/[1.25] xl:text-2xl italic lowercase font-extralight -translate-y-[0.25rem] text-neutral-600 dark:text-neutral-300 tracking-wide" {
                            (haiku[2])
                        }
                    }
                }
            }
        }
    }
}

fn work_section(work: &Work) -> Markup {
    html! {
        figure class="work
            w-full shrink
            max-h-full
            flex flex-col justify-center items-end
            tracking-wide font-light
            " {
            (Image::new(&work.image)
                .class("max-w-full max-h-full block rounded-md object-contain"))
        }
    }
}
