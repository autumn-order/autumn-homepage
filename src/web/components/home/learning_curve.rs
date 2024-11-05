use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaBook, FaGraduationCap, FaHandshake, FaTrafficLight},
    Icon,
};
use manganis::ImageAsset;

#[component]
pub fn LearningCurve() -> Element {
    const EVE_LEARNING_CURVE_IMAGE: ImageAsset =
        manganis::mg!(image("assets/images/join/the-great-curve.jpg")
            .format(ImageType::Avif)
            .size(647, 500));

    rsx! {
        section { class: "flex items-center justify-center",
            div { class: "max-w-[1440px] p-6 w-full h-full flex flex-col items-center",
                div { class: "py-6",
                    h1 { class: "text-center font-bold text-2xl xl:text-4xl pb-4",
                        "Conquer the EVE Learning Curve"
                    }
                    p { class: "sm:text-xl text-center",
                        "EVE's learning curve is steep but worth it, have support & community every step of the way towards realizing your dreams in EVE Online."
                    }
                }
                div { class: "py-6 flex flex-wrap",
                    div { class: "md:w-1/2 pb-6 md:pr-6",
                        img {
                            class: "max-:w-[647px] max-h-[500px]",
                            alt: "The EVE learning curve",
                            src: EVE_LEARNING_CURVE_IMAGE
                        }
                    }
                    ul { class: "md:w-1/2 flex flex-col gap-4",
                        li {
                            div { class: "card card-compact shadow",
                                div { class: "card-body",
                                    h2 { class: "card-title",
                                        div { class: "w-6 h-6",
                                            Icon { icon: FaHandshake, width: 24, height: 24 }
                                        }
                                        span { "Community" }
                                    }
                                    p {
                                        "Begin with the advantage of connections with a vast & highly experienced community ranging from veteran capital pilots, hardcore industrialists, large scale corporation leaders, and those just beginning their EVE journey."
                                    }
                                }
                            }
                        }
                        li {
                            div { class: "card card-compact shadow",
                                div { class: "card-body",
                                    h2 { class: "card-title",
                                        div { class: "w-6 h-6",
                                            Icon { icon: FaGraduationCap, width: 24, height: 24 }
                                        }
                                        span { "Classes" }
                                    }
                                    p {
                                        "Begin with the advantage of connections with a vast & highly experienced community ranging from veteran capital pilots, hardcore industrialists, large scale corporation leaders, and those just beginning their EVE journey."
                                    }
                                }
                            }
                        }
                        li {
                            div { class: "card card-compact shadow",
                                div { class: "card-body",
                                    h2 { class: "card-title",
                                        div { class: "w-6 h-6",
                                            Icon { icon: FaBook, width: 24, height: 24 }
                                        }
                                        span { "Information" }
                                    }
                                    p {
                                        "Begin with the advantage of connections with a vast & highly experienced community ranging from veteran capital pilots, hardcore industrialists, large scale corporation leaders, and those just beginning their EVE journey."
                                    }
                                }
                            }
                        }
                        li {
                            div { class: "card card-compact shadow",
                                div { class: "card-body",
                                    h2 { class: "card-title",
                                        div { class: "w-6 h-6",
                                            Icon { icon: FaTrafficLight, width: 24, height: 24 }
                                        }
                                        span { "Play at Your Pace" }
                                    }
                                    p {
                                        "Begin with the advantage of connections with a vast & highly experienced community ranging from veteran capital pilots, hardcore industrialists, large scale corporation leaders, and those just beginning their EVE journey."
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
