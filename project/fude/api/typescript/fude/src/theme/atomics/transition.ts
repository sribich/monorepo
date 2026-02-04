import { create } from "@stylexjs/stylex"

const ALLOW_MOTION = "@media (prefers-reduced-motion: no-preference)"

export const transitionStyles = create({
    opacity: {
        transitionProperty: {
            default: "none",
            [ALLOW_MOTION]: "opacity",
        },
        transitionTimingFunction: {
            default: "unset",
            [ALLOW_MOTION]: "cubic-bezier(0.4, 0, 0.2, 1)",
        },
        transitionDuration: {
            default: "unset",
            [ALLOW_MOTION]: "250ms",
        },
    },
    color: {
        transitionProperty: {
            default: "none",
            [ALLOW_MOTION]:
                "color, background-color, border-color, text-decoration-color, fill, stroke",
        },
        transitionTimingFunction: {
            default: "unset",
            [ALLOW_MOTION]: "cubic-bezier(0.4, 0, 0.2, 1)",
        },
        transitionDuration: {
            default: "unset",
            [ALLOW_MOTION]: "250ms",
        },
    },
    movement: {
        transitionProperty: {
            default: "none",
            [ALLOW_MOTION]: "transform, translate, scale, left, right, width, top, opacity, margin",
        },
        transitionTimingFunction: {
            default: "unset",
            // [ALLOW_MOTION]: "linear",
            [ALLOW_MOTION]: "cubic-bezier(0.4, 0, 0.2, 1)",
        },
        transitionDuration: {
            default: "unset",
            [ALLOW_MOTION]: "250ms",
        },
    },
})

// color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, --tw-gradient-from, --tw-gradient-via, --tw-gradient-to, opacity, box-shadow, transform, translate, scale, rotate, filter, -webkit-backdrop-filter, backdrop-filter, display, content-visibility, overlay, pointer-events
