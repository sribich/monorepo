import { scope, type } from "arktype"
import { addDays, endOfDay, isAfter, isBefore, isSameDay, isToday, startOfDay } from "date-fns"

import { dateFilterType, parseDate } from "../../../util/arktype"
import { makeProperty } from "../property-definition"

const filterCheckpoints = (() => {
    const getSecondsUntilTomorrow = (date: Date) => {
        const h = date.getHours()
        const m = date.getMinutes()
        const s = date.getSeconds()

        return 86400 - h * 3600 - m * 60 - s
    }

    let today = new Date()

    const dateRanges = {
        today: {
            start: startOfDay(today),
            end: endOfDay(today),
        },
        tomorrow: {
            start: startOfDay(addDays(today, 1)),
            end: endOfDay(addDays(today, 1)),
        },
    }

    const updateRanges = () => {
        dateRanges.today = {
            start: startOfDay(today),
            end: endOfDay(today),
        }
        dateRanges.tomorrow = {
            start: startOfDay(addDays(today, 1)),
            end: endOfDay(addDays(today, 1)),
        }
    }

    const updateTimeout = () => {
        setTimeout(() => {
            today = addDays(today, 1)

            updateRanges()
            updateTimeout()
        }, getSecondsUntilTomorrow(today) * 1000)
    }

    updateTimeout()

    return dateRanges
})()

export const dateFormats = ["Full Date", "MM/DD/YYYY", "DD/MM/YYYY", "YYYY/MM/DD"] as const
export const timeFormats = ["12 hour", "24 hour"] as const

export const dateFilters = scope(
    {
        is: {
            $type: "'date'",
            kind: "'IS'",
            data: dateFilterType,
        },
        is_before: {
            kind: "'IS_BEFORE'",
            data: dateFilterType,
        },
        is_after: {
            kind: "'IS_AFTER'",
            data: dateFilterType,
        },
        is_on_or_before: {
            kind: "'IS_ON_OR_BEFORE'",
            data: dateFilterType,
        },
        is_on_or_after: {
            kind: "'IS_ON_OR_AFTER'",
            data: dateFilterType,
        },
        is_between: {
            kind: "'IS_BETWEEN'",
            data: type([dateFilterType, dateFilterType]),
        },
        is_empty: {
            kind: "'IS_EMPTY'",
            data: "never",
        },
        is_not_empty: {
            kind: "'IS_NOT_EMPTY'",
            data: "never",
        },
        union: "is | is_before | is_after | is_on_or_before | is_on_or_after | is_between | is_empty | is_not_empty",
    },
    {},
).export().union

export interface DateValue {
    date: number
    dateEnd: number | null
}

export const date = makeProperty("date")({
    name: "Date",
    config: {
        default: {
            dateFormat: "Full Date",
            timeFormat: "24 hour",
        },
        type: type({
            dateFormat: "'Full Date' | 'MM/DD/YYYY' | 'DD/MM/YYYY' | 'YYYY/MM/DD'",
            timeFormat: "'12 hour' | '24 hour'",
        }),
        morphs: {},
    },
    field: {
        default: {
            date: null,
            dateEnd: null,
        },
        type: type({
            "date?": parseDate.or(type("null")),
            "dateEnd?": parseDate.or(type("null")),
        }),
        morphs: {},
    },
    filter: {
        type: dateFilters,
        default: {
            kind: "IS",
            data: "today",
        },
        filters: {
            IS: {
                fn: (_property, filter, value) => {
                    if (!value.date) {
                        return false
                    }

                    const filterDate =
                        typeof filter.data === "string"
                            ? filterCheckpoints[filter.data].start
                            : filter.data
                    const valueDate = value.date

                    return (
                        filterDate.getDate() === valueDate.getDate() &&
                        filterDate.getMonth() === valueDate.getMonth() &&
                        filterDate.getFullYear() === valueDate.getFullYear()
                    )
                },
                default: "today",
            },
            IS_BEFORE: {
                fn: (_property, filter, value) => {
                    if (!value.date) {
                        return false
                    }

                    const filterDate =
                        typeof filter.data === "string"
                            ? filterCheckpoints[filter.data].start
                            : filter.data

                    return isBefore(value.date, filterDate)
                },
                default: "today",
            },
            IS_AFTER: {
                fn: (_property, filter, value) => {
                    if (!value.date) {
                        return false
                    }

                    const filterDate =
                        typeof filter.data === "string"
                            ? filterCheckpoints[filter.data].end
                            : filter.data

                    return isAfter(value.date, filterDate)
                },
                default: "today",
            },
            IS_ON_OR_BEFORE: {
                fn: (_property, filter, value) => {
                    if (!value.date) {
                        return false
                    }

                    const filterDate =
                        typeof filter.data === "string"
                            ? filterCheckpoints[filter.data].start
                            : filter.data

                    return isSameDay(value.date, filterDate) || isBefore(value.date, filterDate)
                },
                default: "today",
            },
            IS_ON_OR_AFTER: {
                fn: (_property, filter, value) => {
                    if (!value.date) {
                        return false
                    }

                    const filterDate =
                        typeof filter.data === "string"
                            ? filterCheckpoints[filter.data].end
                            : filter.data

                    return isSameDay(value.date, filterDate) || isAfter(value.date, filterDate)
                },
                default: "today",
            },
            IS_BETWEEN: {
                fn: (_property, filter, value) => {
                    if (!value.date) {
                        return false
                    }
                    return false
                },
                default: ["today", "today"],
            },
            IS_EMPTY: {
                fn: (_property, filter, value) => {
                    return !value.date
                },
                default: "" as never,
            },
            IS_NOT_EMPTY: {
                fn: (_property, filter, value) => {
                    return !!value.date
                },
                default: "" as never,
            },
        },
    },
})
