use dioxus::{logger::tracing::info, prelude::*};
//use dioxus_sdk::utils::timing::{use_debounce, use_interval};
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};

const CSS: Asset = asset!("/assets/slideshow.css");

#[derive(Debug, Clone, PartialEq, Props)]
pub struct Slideshow {
    slides: Vec<&'static str>,
    slide_duration: u64,
    anim_duration: u64,
    enter: &'static str,
    leave: &'static str,
    enter_back: &'static str,
    leave_back: &'static str,
    style: Option<String>,
}

impl Slideshow {
    pub fn new(slides: Vec<&'static str>) -> Self {
        Self {
            slides,
            enter: "slideInLeft",
            leave: "slideOutRight",
            enter_back: "slideInRight",
            leave_back: "slideOutLeft",
            slide_duration: 3_000,
            style: Some("".to_string()),
            anim_duration: 3_000,
        }
    }

    pub fn enter(mut self, enter: &'static str) -> Self {
        self.enter = enter;
        self.enter_back = enter;
        self
    }

    pub fn leave(mut self, leave: &'static str) -> Self {
        self.leave = leave;
        self.leave_back = leave;
        self
    }

    pub fn enter_back(mut self, enter_back: &'static str) -> Self {
        self.enter_back = enter_back;
        self
    }

    pub fn leave_back(mut self, leave_back: &'static str) -> Self {
        self.leave_back = leave_back;
        self
    }

    pub fn set_current_fixed(mut self) -> Self {
        self.leave = "";
        self.leave_back = "";
        self
    }

    pub fn slide_duration(mut self, slide_duration: u64) -> Self {
        self.slide_duration = slide_duration;
        self
    }

    pub fn anim_duration(mut self, anim_duration: u64) -> Self {
        let dur = format!("animation-duration:{}ms;", anim_duration);
        self.style.as_mut().unwrap().push_str(dur.as_str());
        self
    }

    pub fn render(self) -> Element {
        rsx! {
            {SlideshowView(self)}
        }
    }
}

//#[component]
pub fn SlideshowView(props: Slideshow) -> Element {
    let last = Rc::new(props.slides.len() - 1);
    let mut forward = use_signal(|| true);
    let mut in_anim_class = use_signal(|| "");
    let mut out_anim_class = use_signal(|| "");

    let mut current_slide: Signal<isize> = use_signal(|| -1);

    let next_slide: Memo<isize> = use_memo({
        let last = last.clone();
        move || {
            if forward() {
                if current_slide() == *last as isize {
                    0
                } else {
                    current_slide() + 1
                }
            } else {
                if current_slide() == 0 {
                    *last as isize
                } else {
                    current_slide() - 1
                }
            }
        }
    });

    let move_slide = Rc::new(RefCell::new({
        let last = last.clone();
        move || {
            current_slide.with_mut(|curr| {
                *curr = if forward() {
                    if *curr == *last as isize {
                        0
                    } else {
                        *curr + 1
                    }
                } else {
                    if *curr == 0 {
                        *last as isize
                    } else {
                        *curr - 1
                    }
                };
            });
        }
    }));

    let mut interval = use_interval(Duration::from_millis(props.slide_duration), {
        let move_slide = move_slide.clone();
        move || {
            move_slide.borrow_mut()();
            in_anim_class.set(props.enter);
            out_anim_class.set(props.leave);
        }
    });

    // let mut interval = spawn({
    //     let move_slide = move_slide.clone();
    //     async move {
    //         loop {
    //             gloo_timers::future::sleep(Duration::from_secs(3)).await;
    //             move_slide.borrow_mut()();
    //             in_anim_class.set("slideInLeft");
    //             out_anim_class.set("slideOutRight");
    //         }
    //     }
    // });

    let mut move_slide_manual = {
        let move_slide = move_slide.clone();

        move || {
            interval.cancel();
            if !forward() {
                forward.set(true);
                move_slide.borrow_mut()();
                move_slide.borrow_mut()();
            }
            move_slide.borrow_mut()();
            in_anim_class.set(props.enter);
            out_anim_class.set(props.leave);
        }
    };

    let mut back = {
        let move_slide = move_slide.clone();
        move || {
            interval.cancel();
            if forward() {
                forward.set(false);
                move_slide.borrow_mut()();
                move_slide.borrow_mut()();
            }
            move_slide.borrow_mut()();
            in_anim_class.set(props.enter_back);
            out_anim_class.set(props.leave_back);
        }
    };

    rsx! {
        document::Stylesheet { href: "{CSS}" }
        div {
            class: "slideshow",
            width: "1000px",
            height: "500px",
            max_width: "100%",
            div { class: "bg absolute" }
            for (id , src) in props.slides.iter().enumerate() {
                if id as isize == current_slide() {
                    Slide {
                        src,
                        id,
                        anim_class: out_anim_class,
                        anim_style: props.style.as_ref().unwrap(),
                        z_index: 0,
                    }
                } else if id as isize == next_slide() {
                    Slide {
                        src,
                        id,
                        anim_class: in_anim_class,
                        anim_style: props.style.as_ref().unwrap(),
                        z_index: 1,
                    }
                }
            }
        }
        button { onclick: move |_| back(), "<<<" }
        button { onclick: move |_| move_slide_manual(), ">>>" }

        p { "{next_slide()}" }
    }
}

#[component]
pub fn Slide(
    src: String,
    id: usize,
    anim_class: String,
    anim_style: String,
    z_index: usize,
) -> Element {
    rsx! {
        div { class: "absolute {anim_class}", style: anim_style, z_index,
            img { src, loading: "lazy" }
        }
    }
}

//---------------------------------------
use dioxus::prelude::{use_hook, Callback, Writable};

#[derive(Clone, PartialEq, Copy)]
pub struct UseInterval {
    inner: dioxus::prelude::Signal<InnerUseInterval>,
}

struct InnerUseInterval {
    pub(crate) interval: Option<dioxus::prelude::Task>,
}

impl Drop for InnerUseInterval {
    fn drop(&mut self) {
        if let Some(interval) = self.interval.take() {
            interval.cancel();
        }
    }
}

impl UseInterval {
    /// Cancel the interval
    pub fn cancel(&mut self) {
        if let Some(interval) = self.inner.write().interval.take() {
            interval.cancel();
        }
    }

    pub fn pause(&self) {
        if let Some(interval) = self.inner.read().interval {
            interval.pause();
        }
    }

    pub fn resume(&self) {
        if let Some(interval) = self.inner.read().interval {
            interval.resume();
        }
    }

    pub fn wake(&self) {
        if let Some(interval) = self.inner.read().interval {
            interval.wake();
        }
    }
}

/// Repeatedly calls a function every a certain period.
pub fn use_interval(period: Duration, mut action: impl FnMut() + 'static) -> UseInterval {
    let inner = use_hook(|| {
        let callback = Callback::new(move |()| {
            action();
        });

        dioxus::prelude::Signal::new(InnerUseInterval {
            interval: Some(dioxus::prelude::spawn(async move {
                loop {
                    gloo_timers::future::sleep(period).await;

                    callback.call(());
                }
            })),
        })
    });

    UseInterval { inner }
}
