import { randomUUID } from "crypto"

import { type } from "arktype"

import { rainbowColor } from "../../../util/color"
import { makeProperty } from "../property-definition"

export type SelectFieldConfig = (typeof select)["config"]["default"]
export type SelectFieldValue = (typeof select)["field"]["default"]

export type SelectProperty = (typeof select)["config"]["schema"]["infer"]
export type SelectPropertyOption = (typeof select)["config"]["type"]["infer"]["options"][number]

const selectOption = type({
    id: "string",
    name: "string",
    color: "string",
})

export const select = makeProperty("select")({
    name: "Select",
    config: {
        default: {
            sort: "manual",
            options: [],
        },
        type: type({
            sort: "'manual' | 'asc' | 'desc'",
            // TODO: We might want to have some sort of "cache" field which we can use to store
            // TODO: the options array in a id => { name, color } format for faster lookups.
            options: type(selectOption, "[]"),
        }),
        morphs: {
            renameOption: (id: string, name: string) => {
                return (incomingConfig) => {
                    const config = incomingConfig.options.find((it) => it.id === id)

                    if (config) {
                        config.name = name
                    }
                }
            },
            color: (id: string, color: string) => {
                return (incomingConfig) => {
                    const config = incomingConfig.options.find((it) => it.id === id)

                    if (config) {
                        config.color = color
                    }
                }
            },
            rewriteOptions: (options: (typeof selectOption.infer)[]) => {
                return (incomingConfig) => {
                    incomingConfig.options = options
                }
            },
            addOption: (optionName: string) => (incomingConfig) => {
                if (incomingConfig.options.find((it) => it.name === optionName)) {
                    return incomingConfig
                }

                incomingConfig.options.push({
                    id: randomUUID(),
                    name: optionName,
                    color: rainbowColor(24, incomingConfig.options.length),
                })

                return incomingConfig
            },
        },
    },
    field: {
        default: null,
        type: type("string | null"),
        morphs: {},
    },
    filter: {
        type: type("never"),
        filters: {},
    },
})
