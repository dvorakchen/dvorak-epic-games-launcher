use leptos::*;

const SVG_DEFAULT_CSS: &'static str = "fill-inherit w-inherit h-inherit stroke-inherit";

pub enum IconTypes {
    Earth,
    Tag,
    Grid,
    UnrealEngine,
}

impl IntoView for IconTypes {
    fn into_view(self) -> View {
        match self {
            IconTypes::Earth => Earth().into_view(),
            IconTypes::Tag => Tag().into_view(),
            IconTypes::Grid => Grid().into_view(),
            IconTypes::UnrealEngine => UnrealEngine().into_view(),
        }
    }
}

#[component]
pub fn Gear() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="currentColor"
            width="16"
            height="16"
            class=SVG_DEFAULT_CSS
            viewBox="0 0 16 16"
        >
            <path d="M9.405 1.05c-.413-1.4-2.397-1.4-2.81 0l-.1.34a1.464 1.464 0 0 1-2.105.872l-.31-.17c-1.283-.698-2.686.705-1.987 1.987l.169.311c.446.82.023 1.841-.872 2.105l-.34.1c-1.4.413-1.4 2.397 0 2.81l.34.1a1.464 1.464 0 0 1 .872 2.105l-.17.31c-.698 1.283.705 2.686 1.987 1.987l.311-.169a1.464 1.464 0 0 1 2.105.872l.1.34c.413 1.4 2.397 1.4 2.81 0l.1-.34a1.464 1.464 0 0 1 2.105-.872l.31.17c1.283.698 2.686-.705 1.987-1.987l-.169-.311a1.464 1.464 0 0 1 .872-2.105l.34-.1c1.4-.413 1.4-2.397 0-2.81l-.34-.1a1.464 1.464 0 0 1-.872-2.105l.17-.31c.698-1.283-.705-2.686-1.987-1.987l-.311.169a1.464 1.464 0 0 1-2.105-.872zM8 10.93a2.929 2.929 0 1 1 0-5.86 2.929 2.929 0 0 1 0 5.858z"></path>
        </svg>
    }
}

#[component]
pub fn ChevronLeft() -> impl IntoView {
    view! {
        <svg
            class=SVG_DEFAULT_CSS
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path
                fill-rule="evenodd"
                d="M11.354 1.646a.5.5 0 0 1 0 .708L5.707 8l5.647 5.646a.5.5 0 0 1-.708.708l-6-6a.5.5 0 0 1 0-.708l6-6a.5.5 0 0 1 .708 0"
            ></path>
        </svg>
    }
}

#[component]
pub fn ChevronRight() -> impl IntoView {
    view! {
        <svg
            class=SVG_DEFAULT_CSS
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path
                fill-rule="evenodd"
                d="M4.646 1.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L10.293 8 4.646 2.354a.5.5 0 0 1 0-.708"
            ></path>
        </svg>
    }
}
#[component]
pub fn ChevronDown() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            class=SVG_DEFAULT_CSS
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path
                fill-rule="evenodd"
                d="M1.646 4.646a.5.5 0 0 1 .708 0L8 10.293l5.646-5.647a.5.5 0 0 1 .708.708l-6 6a.5.5 0 0 1-.708 0l-6-6a.5.5 0 0 1 0-.708"
            ></path>
        </svg>
    }
}

#[component]
pub fn Earth() -> impl IntoView {
    view! {
        <svg
            class=SVG_DEFAULT_CSS
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path d="M8 0a8 8 0 1 0 0 16A8 8 0 0 0 8 0M4.882 1.731a.48.48 0 0 0 .14.291.487.487 0 0 1-.126.78l-.291.146a.7.7 0 0 0-.188.135l-.48.48a1 1 0 0 1-1.023.242l-.02-.007a1 1 0 0 0-.462-.04 7 7 0 0 1 2.45-2.027m-3 9.674.86-.216a1 1 0 0 0 .758-.97v-.184a1 1 0 0 1 .445-.832l.04-.026a1 1 0 0 0 .152-1.54L3.121 6.621a.414.414 0 0 1 .542-.624l1.09.818a.5.5 0 0 0 .523.047.5.5 0 0 1 .724.447v.455a.8.8 0 0 0 .131.433l.795 1.192a1 1 0 0 1 .116.238l.73 2.19a1 1 0 0 0 .949.683h.058a1 1 0 0 0 .949-.684l.73-2.189a1 1 0 0 1 .116-.238l.791-1.187A.45.45 0 0 1 11.743 8c.16 0 .306.084.392.218.557.875 1.63 2.282 2.365 2.282l.04-.001a7.003 7.003 0 0 1-12.658.905Z"></path>
        </svg>
    }
}

#[component]
pub fn DashCircle() -> impl IntoView {
    view! {
        <svg
            class=SVG_DEFAULT_CSS
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14m0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16"></path>
            <path d="M4 8a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 0 1h-7A.5.5 0 0 1 4 8"></path>
        </svg>
    }
}

#[component]
pub fn PencilSquare() -> impl IntoView {
    view! {
        <svg
            class=SVG_DEFAULT_CSS
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path d="M15.502 1.94a.5.5 0 0 1 0 .706L14.459 3.69l-2-2L13.502.646a.5.5 0 0 1 .707 0l1.293 1.293zm-1.75 2.456-2-2L4.939 9.21a.5.5 0 0 0-.121.196l-.805 2.414a.25.25 0 0 0 .316.316l2.414-.805a.5.5 0 0 0 .196-.12l6.813-6.814z"></path>
            <path
                fill-rule="evenodd"
                d="M1 13.5A1.5 1.5 0 0 0 2.5 15h11a1.5 1.5 0 0 0 1.5-1.5v-6a.5.5 0 0 0-1 0v6a.5.5 0 0 1-.5.5h-11a.5.5 0 0 1-.5-.5v-11a.5.5 0 0 1 .5-.5H9a.5.5 0 0 0 0-1H2.5A1.5 1.5 0 0 0 1 2.5z"
            ></path>
        </svg>
    }
}

#[component]
pub fn Eye() -> impl IntoView {
    view! {
        <svg
            class=SVG_DEFAULT_CSS
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path d="M16 8s-3-5.5-8-5.5S0 8 0 8s3 5.5 8 5.5S16 8 16 8M1.173 8a13 13 0 0 1 1.66-2.043C4.12 4.668 5.88 3.5 8 3.5s3.879 1.168 5.168 2.457A13 13 0 0 1 14.828 8q-.086.13-.195.288c-.335.48-.83 1.12-1.465 1.755C11.879 11.332 10.119 12.5 8 12.5s-3.879-1.168-5.168-2.457A13 13 0 0 1 1.172 8z"></path>
            <path d="M8 5.5a2.5 2.5 0 1 0 0 5 2.5 2.5 0 0 0 0-5M4.5 8a3.5 3.5 0 1 1 7 0 3.5 3.5 0 0 1-7 0"></path>
        </svg>
    }
}

#[component]
pub fn EyeSlash() -> impl IntoView {
    view! {
        <svg
            class=SVG_DEFAULT_CSS
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path d="M13.359 11.238C15.06 9.72 16 8 16 8s-3-5.5-8-5.5a7 7 0 0 0-2.79.588l.77.771A6 6 0 0 1 8 3.5c2.12 0 3.879 1.168 5.168 2.457A13 13 0 0 1 14.828 8q-.086.13-.195.288c-.335.48-.83 1.12-1.465 1.755q-.247.248-.517.486z"></path>
            <path d="M11.297 9.176a3.5 3.5 0 0 0-4.474-4.474l.823.823a2.5 2.5 0 0 1 2.829 2.829zm-2.943 1.299.822.822a3.5 3.5 0 0 1-4.474-4.474l.823.823a2.5 2.5 0 0 0 2.829 2.829"></path>
            <path d="M3.35 5.47q-.27.24-.518.487A13 13 0 0 0 1.172 8l.195.288c.335.48.83 1.12 1.465 1.755C4.121 11.332 5.881 12.5 8 12.5c.716 0 1.39-.133 2.02-.36l.77.772A7 7 0 0 1 8 13.5C3 13.5 0 8 0 8s.939-1.721 2.641-3.238l.708.709zm10.296 8.884-12-12 .708-.708 12 12z"></path>
        </svg>
    }
}

#[component]
pub fn ArrowRepeat() -> impl IntoView {
    view! {
        <svg
            class=SVG_DEFAULT_CSS
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path d="M11.534 7h3.932a.25.25 0 0 1 .192.41l-1.966 2.36a.25.25 0 0 1-.384 0l-1.966-2.36a.25.25 0 0 1 .192-.41m-11 2h3.932a.25.25 0 0 0 .192-.41L2.692 6.23a.25.25 0 0 0-.384 0L.342 8.59A.25.25 0 0 0 .534 9"></path>
            <path
                fill-rule="evenodd"
                d="M8 3c-1.552 0-2.94.707-3.857 1.818a.5.5 0 1 1-.771-.636A6.002 6.002 0 0 1 13.917 7H12.9A5 5 0 0 0 8 3M3.1 9a5.002 5.002 0 0 0 8.757 2.182.5.5 0 1 1 .771.636A6.002 6.002 0 0 1 2.083 9z"
            ></path>
        </svg>
    }
}

#[component]
pub fn X() -> impl IntoView {
    view! {
        <svg
            class=SVG_DEFAULT_CSS
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708"></path>
        </svg>
    }
}

#[component]
pub fn XCircle() -> impl IntoView {
    view! {
        <svg
            class=SVG_DEFAULT_CSS
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14m0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16"></path>
            <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708"></path>
        </svg>
    }
}

#[component]
pub fn Epic() -> impl IntoView {
    view! {
        <svg
            class=SVG_DEFAULT_CSS
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 32 32"
            fill-rule="evenodd"
            height="32"
            width="32"
        >
            <path d="M12.349 21l-.39.982h.773L12.349 21zm2.29-10.224V7.909c0-.457-.21-.668-.648-.668h-.713v4.204h.713c.438 0 .648-.212.648-.668zm11.698-9.019H5.36c-1.702 0-2.328.627-2.328 2.328v20.526l.025.537c.038.372.047.732.392 1.14a7.23 7.23 0 0 0 .387.302l.533.248 10.329 4.327c.537.245.76.342 1.15.333h.003c.39.008.613-.088 1.15-.333l10.329-4.327.533-.248.387-.302c.345-.41.353-.77.392-1.14a5.28 5.28 0 0 0 .025-.537V4.086c0-1.702-.628-2.328-2.328-2.328h-.002zm-9.181 3.952h1.735v11.404h-1.735V5.709zm.117 14.181h1.01v3.45h-.952v-1.982l-.882 1.35h-.02l-.877-1.34v1.972h-.937v-3.45h1.01l.823 1.337.823-1.337zm-5.73-14.181h2.723c1.41 0 2.108.7 2.108 2.118v3.03c0 1.417-.697 2.118-2.108 2.118h-.988v4.139h-1.735V5.709zm-4.729 0h3.859v1.58H8.549v3.225h2.043v1.58H8.549v3.438h2.157v1.58H6.814V5.709zm3.317 17.169c-.365.3-.873.532-1.498.532-1.075 0-1.878-.74-1.878-1.785v-.01c0-1.005.788-1.795 1.858-1.795.607 0 1.035.187 1.4.503l-.562.675c-.247-.207-.493-.325-.833-.325-.498 0-.882.418-.882.947v.01a.91.91 0 0 0 .937.957c.232 0 .41-.05.552-.143v-.418h-.68v-.7h1.587v1.553zm1.765-3.012h.922l1.468 3.475h-1.025l-.252-.617h-1.332l-.247.617h-1.005l1.468-3.475h.002zm3.874 9.231l-4.862-1.672h9.931l-5.069 1.672zm5.947-5.755h-2.8v-3.45h2.775v.813h-1.828v.523h1.657v.755h-1.657v.547h1.854v.813-.002zm-1.907-8.249V7.729c0-1.417.697-2.118 2.108-2.118h.843c1.41 0 2.092.685 2.092 2.102v2.33h-1.702V7.811c0-.457-.212-.668-.648-.668h-.292c-.453 0-.665.212-.665.668v7.2c0 .457.212.668.665.668h.325c.438 0 .648-.212.648-.668v-2.573h1.702v2.655c0 1.417-.697 2.119-2.108 2.119h-.86c-1.41 0-2.108-.7-2.108-2.119zm5.165 7.184c0 .705-.557 1.123-1.395 1.123-.612 0-1.193-.192-1.617-.572l.532-.637a1.77 1.77 0 0 0 1.118.413c.257 0 .395-.088.395-.237v-.01c0-.143-.113-.222-.582-.33-.735-.168-1.302-.375-1.302-1.085v-.01c0-.642.508-1.105 1.337-1.105.587 0 1.045.158 1.42.458l-.478.675c-.315-.222-.66-.34-.967-.34-.232 0-.345.098-.345.222v.01c0 .158.118.227.597.335.793.173 1.287.428 1.287 1.075v.01.003z"></path>
        </svg>
    }
}

#[component]
pub fn Tag() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            class=SVG_DEFAULT_CSS
            viewBox="0 0 16 16"
        >
            <path d="M2 1a1 1 0 0 0-1 1v4.586a1 1 0 0 0 .293.707l7 7a1 1 0 0 0 1.414 0l4.586-4.586a1 1 0 0 0 0-1.414l-7-7A1 1 0 0 0 6.586 1zm4 3.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0"></path>
        </svg>
    }
}

#[component]
pub fn Grid() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            class=SVG_DEFAULT_CSS
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path d="M1 2.5A1.5 1.5 0 0 1 2.5 1h3A1.5 1.5 0 0 1 7 2.5v3A1.5 1.5 0 0 1 5.5 7h-3A1.5 1.5 0 0 1 1 5.5zm8 0A1.5 1.5 0 0 1 10.5 1h3A1.5 1.5 0 0 1 15 2.5v3A1.5 1.5 0 0 1 13.5 7h-3A1.5 1.5 0 0 1 9 5.5zm-8 8A1.5 1.5 0 0 1 2.5 9h3A1.5 1.5 0 0 1 7 10.5v3A1.5 1.5 0 0 1 5.5 15h-3A1.5 1.5 0 0 1 1 13.5zm8 0A1.5 1.5 0 0 1 10.5 9h3a1.5 1.5 0 0 1 1.5 1.5v3a1.5 1.5 0 0 1-1.5 1.5h-3A1.5 1.5 0 0 1 9 13.5z"></path>
        </svg>
    }
}

#[component]
pub fn UnrealEngine() -> impl IntoView {
    view! {
        <svg
            class=SVG_DEFAULT_CSS
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 42 42"
            fill-rule="evenodd"
        >
            <path d="M33.479 24.025c-.362 1.746-1.972 6.225-7.107 8.651l-2.062-2.319-3.48 3.496c-2.229-.006-6.874-.794-10.155-5.209 0 0 .652.206 1.137.215.569.01 1.187-.198 1.187-1.155v-9.439c.062-.784-.693-1.844-1.955-1.556-1.611.368-2.898 4.387-2.898 4.387-.004-4.855 2.491-8.161 4.425-9.795 2.403-2.032 4.746-2.73 6.526-3.011-1.758 1.003-2.747 2.637-2.747 4.008 0 2.202 1.329 1.941 1.723 1.616v12.739s.079.205.261.448c.265.351.751.782 1.538.782 1.331 0 3.058-1.519 3.058-1.519V16.05c0-1.05-.792-2.315-1.583-2.751 0 0 1.465-.264 2.599.607a9.56 9.56 0 0 1 .676-.744c2.635-2.59 5.123-3.324 7.194-3.697h.003s-3.769 2.959-3.769 6.928l.076 10.153c1.397 1.352 3.474-.598 5.352-2.52zM21 41c11.046 0 20-8.954 20-20S32.046 1 21 1 1 9.954 1 21s8.954 20 20 20zm0 1c11.598 0 21-9.402 21-21S32.598 0 21 0 0 9.402 0 21s9.402 21 21 21z"></path>
        </svg>
    }
}

#[component]
pub fn Search() -> impl IntoView {
    view! {
        <svg
            class=SVG_DEFAULT_CSS
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001q.044.06.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1 1 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0"></path>
        </svg>
    }
}

#[component]
pub fn People() -> impl IntoView {
    view! {
        <svg class=SVG_DEFAULT_CSS width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
            <path d="M7 14s-1 0-1-1 1-4 5-4 5 3 5 4-1 1-1 1zm4-6a3 3 0 1 0 0-6 3 3 0 0 0 0 6m-5.784 6A2.24 2.24 0 0 1 5 13c0-1.355.68-2.75 1.936-3.72A6.3 6.3 0 0 0 5 9c-4 0-5 3-5 4s1 1 1 1zM4.5 8a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5"></path>
        </svg>
    }
}

#[component]
pub fn BoxArrowUpRight() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            class=SVG_DEFAULT_CSS
            viewBox="0 0 16 16"
        >
            <path
                fill-rule="evenodd"
                d="M8.636 3.5a.5.5 0 0 0-.5-.5H1.5A1.5 1.5 0 0 0 0 4.5v10A1.5 1.5 0 0 0 1.5 16h10a1.5 1.5 0 0 0 1.5-1.5V7.864a.5.5 0 0 0-1 0V14.5a.5.5 0 0 1-.5.5h-10a.5.5 0 0 1-.5-.5v-10a.5.5 0 0 1 .5-.5h6.636a.5.5 0 0 0 .5-.5"
            ></path>
            <path
                fill-rule="evenodd"
                d="M16 .5a.5.5 0 0 0-.5-.5h-5a.5.5 0 0 0 0 1h3.793L6.146 9.146a.5.5 0 1 0 .708.708L15 1.707V5.5a.5.5 0 0 0 1 0z"
            ></path>
        </svg>
    }
}

#[component]
pub fn PersonPlus() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            class=SVG_DEFAULT_CSS
            viewBox="0 0 16 16"
        >
            <path d="M1 14s-1 0-1-1 1-4 6-4 6 3 6 4-1 1-1 1zm5-6a3 3 0 1 0 0-6 3 3 0 0 0 0 6"></path>
            <path
                fill-rule="evenodd"
                d="M13.5 5a.5.5 0 0 1 .5.5V7h1.5a.5.5 0 0 1 0 1H14v1.5a.5.5 0 0 1-1 0V8h-1.5a.5.5 0 0 1 0-1H13V5.5a.5.5 0 0 1 .5-.5"
            ></path>
        </svg>
    }
}

#[component]
pub fn Play() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            class=SVG_DEFAULT_CSS
            viewBox="0 0 16 16"
        >
            <path d="m11.596 8.697-6.363 3.692c-.54.313-1.233-.066-1.233-.697V4.308c0-.63.692-1.01 1.233-.696l6.363 3.692a.802.802 0 0 1 0 1.393"></path>
        </svg>
    }
}

#[component]
pub fn PlusCircle() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            class=SVG_DEFAULT_CSS
            viewBox="0 0 16 16"
        >
            <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14m0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16"></path>
            <path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4"></path>
        </svg>
    }
}

#[component]
pub fn ListUl() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            class=SVG_DEFAULT_CSS
            viewBox="0 0 16 16"
        >
            <path
                fill-rule="evenodd"
                d="M5 11.5a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 0 1h-9a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 0 1h-9a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 0 1h-9a.5.5 0 0 1-.5-.5m-3 1a1 1 0 1 0 0-2 1 1 0 0 0 0 2m0 4a1 1 0 1 0 0-2 1 1 0 0 0 0 2m0 4a1 1 0 1 0 0-2 1 1 0 0 0 0 2"
            ></path>
        </svg>
    }
}

#[component]
pub fn ThreeDots() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            class=SVG_DEFAULT_CSS
            viewBox="0 0 16 16"
        >
            <path d="M3 9.5a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3m5 0a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3m5 0a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3"></path>
        </svg>
    }
}

#[component]
pub fn Download() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            class=SVG_DEFAULT_CSS
            viewBox="0 0 16 16"
        >
            <path d="M.5 9.9a.5.5 0 0 1 .5.5v2.5a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-2.5a.5.5 0 0 1 1 0v2.5a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2v-2.5a.5.5 0 0 1 .5-.5"></path>
            <path d="M7.646 11.854a.5.5 0 0 0 .708 0l3-3a.5.5 0 0 0-.708-.708L8.5 10.293V1.5a.5.5 0 0 0-1 0v8.793L5.354 8.146a.5.5 0 1 0-.708.708z"></path>
        </svg>
    }
}

#[component]
pub fn ThumbsUp() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            class=SVG_DEFAULT_CSS
            viewBox="0 0 16 16"
        >
            <path d="M6.956 1.745C7.021.81 7.908.087 8.864.325l.261.066c.463.116.874.456 1.012.965.22.816.533 2.511.062 4.51a10 10 0 0 1 .443-.051c.713-.065 1.669-.072 2.516.21.518.173.994.681 1.2 1.273.184.532.16 1.162-.234 1.733q.086.18.138.363c.077.27.113.567.113.856s-.036.586-.113.856c-.039.135-.09.273-.16.404.169.387.107.819-.003 1.148a3.2 3.2 0 0 1-.488.901c.054.152.076.312.076.465 0 .305-.089.625-.253.912C13.1 15.522 12.437 16 11.5 16H8c-.605 0-1.07-.081-1.466-.218a4.8 4.8 0 0 1-.97-.484l-.048-.03c-.504-.307-.999-.609-2.068-.722C2.682 14.464 2 13.846 2 13V9c0-.85.685-1.432 1.357-1.615.849-.232 1.574-.787 2.132-1.41.56-.627.914-1.28 1.039-1.639.199-.575.356-1.539.428-2.59z"></path>
        </svg>
    }
}

#[component]
pub fn ThumbsDown() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            class=SVG_DEFAULT_CSS
            viewBox="0 0 16 16"
        >
            <path d="M6.956 14.534c.065.936.952 1.659 1.908 1.42l.261-.065a1.38 1.38 0 0 0 1.012-.965c.22-.816.533-2.512.062-4.51q.205.03.443.051c.713.065 1.669.071 2.516-.211.518-.173.994-.68 1.2-1.272a1.9 1.9 0 0 0-.234-1.734c.058-.118.103-.242.138-.362.077-.27.113-.568.113-.856 0-.29-.036-.586-.113-.857a2 2 0 0 0-.16-.403c.169-.387.107-.82-.003-1.149a3.2 3.2 0 0 0-.488-.9c.054-.153.076-.313.076-.465a1.86 1.86 0 0 0-.253-.912C13.1.757 12.437.28 11.5.28H8c-.605 0-1.07.08-1.466.217a4.8 4.8 0 0 0-.97.485l-.048.029c-.504.308-.999.61-2.068.723C2.682 1.815 2 2.434 2 3.279v4c0 .851.685 1.433 1.357 1.616.849.232 1.574.787 2.132 1.41.56.626.914 1.28 1.039 1.638.199.575.356 1.54.428 2.591"></path>
        </svg>
    }
}
