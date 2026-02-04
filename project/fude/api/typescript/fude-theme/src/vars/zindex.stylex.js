import { defineVars } from "@stylexjs/stylex"

/**
 * Variables for managing stacking contexts.
 *
 * @see https://developer.mozilla.org/en-US/docs/Web/CSS/z-index
 */
export const zIndex = defineVars({
    /**
     * The box does not establish a new local stacking context and
     * the stack level of the generated box is 0.
     */
    inherit: "auto",
    behind5: "-5",
    behind4: "-4",
    behind3: "-3",
    behind2: "-2",
    behind1: "-1",
    default: "0",
    infront1: "1",
    infront2: "2",
    infront3: "3",
    infront4: "4",
    infront5: "5",
    infrontImportant: "100000",
})
