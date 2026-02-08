type Values =
    | Array<Values>
    | Record<string, boolean>
    | string
    | number
    | bigint
    | boolean
    | null
    | undefined

const toValue = (arg: Values): string | null => {
    if (typeof arg === "string" || typeof arg === "number") {
        return String(arg)
    } else if (typeof arg === "object") {
        let output = ""

        if (Array.isArray(arg)) {
            for (let i = 0; i < arg.length; i++) {
                if (arg[i]) {
                    const value = toValue(arg[i])

                    if (value) {
                        // biome-ignore lint/suspicious/noAssignInExpressions: In expression position, but used as conditional assignment.
                        output && (output += " ")
                        output += value
                    }
                }
            }
        } else {
            for (const key in arg) {
                if (arg[key]) {
                    // biome-ignore lint/suspicious/noAssignInExpressions: In expression position, but used as conditional assignment.
                    output && (output += " ")
                    output += key
                }
            }
        }

        return output
    }

    return null
}

export const clsx = (...args: Array<Values>): string => {
    let output = ""

    for (let i = 0; i < args.length; i++) {
        const arg = args[i]

        if (arg) {
            const value = toValue(arg)

            if (value) {
                // biome-ignore lint/suspicious/noAssignInExpressions: In expression position, but used as conditional assignment.
                output && (output += " ")
                output += value
            }
        }
    }

    return output
}
