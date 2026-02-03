import { createConsola, type ConsolaInstance } from "consola"

export const logger: ConsolaInstance = createConsola({
    fancy: true,
    level: 3,
    formatOptions: {
        colors: true,
        compact: false,
    },
})
