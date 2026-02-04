///
///
///
import {
    CalendarDate,
    createCalendar,
    getDayOfWeek,
    getWeeksInMonth,
    isSameDay,
    isSameMonth,
} from "@internationalized/date"
import type { FocusableElement } from "@react-types/shared"
import { ChevronLeft, ChevronRight } from "lucide-react"
import { type DOMAttributes, type RefObject, useMemo, useRef } from "react"
import {
    type AriaButtonProps,
    type AriaCalendarCellProps,
    type AriaCalendarGridProps,
    type AriaCalendarProps,
    type AriaRangeCalendarProps,
    type DateValue,
    mergeProps,
    useCalendar,
    useCalendarCell,
    useCalendarGrid,
    useDateFormatter,
    useFocusRing,
    useHover,
    useLocale,
    useRangeCalendar,
} from "react-aria"
import {
    type CalendarState,
    type RangeCalendarState,
    useCalendarState,
    useRangeCalendarState,
} from "react-stately"

import { createControlledContext, createGenericContext } from "../../hooks/context"
import { useObjectRef } from "../../hooks/useObjectRef"
import { type VariantProps, useStyles } from "../../theme/props"
import { Button } from "../Button/Button"
import { DatePickerStyleProvider, datePickerStyles, useDatePickerStyles } from "./DatePicker.styles"

////////////////////////////////////////////////////////////////////////////////
/// Utils
////////////////////////////////////////////////////////////////////////////////
export const [useDatePickerContext, DatePickerContext] = createControlledContext<
    DatePickerProps<any>,
    HTMLDivElement
>()

////////////////////////////////////////////////////////////////////////////////
/// DatePicker
////////////////////////////////////////////////////////////////////////////////
export interface DatePickerProps<T extends DateValue>
    extends AriaCalendarProps<T>,
        VariantProps<typeof datePickerStyles> {
    ref?: RefObject<HTMLDivElement>
    /**
     * Renders multiple months at one time.
     * @default 1
     */
    visibleMonths?: 1 | 2 | 3
    // withRange?: boolean
}

export const DatePicker = <T extends DateValue>(props: DatePickerProps<T>) => {
    const { locale } = useLocale()

    const calendarRef = useObjectRef(props.ref)

    // We must memo this to prevent the useCalendarState call from
    // making unnecessary state changes
    const visibleMonths = props.visibleMonths ?? 1
    const visibleDuration = useMemo(() => ({ months: visibleMonths }), [visibleMonths])

    // TODO: Test performance characteristics of memoizing the creation
    //       of the object.
    const state = useCalendarState({
        ...props,
        locale,
        createCalendar,
        visibleDuration,
    })

    const { calendarProps, errorMessageProps, nextButtonProps, prevButtonProps } = useCalendar(
        props,
        state,
    )

    const style = useStyles(datePickerStyles, props)

    return (
        <DatePickerStyleProvider value={style}>
            <CalendarContainer
                calendarProps={calendarProps}
                calendarRef={calendarRef}
                errorMessageProps={errorMessageProps}
                nextButtonProps={nextButtonProps}
                prevButtonProps={prevButtonProps}
                state={state}
                visibleMonths={visibleMonths}
            />
        </DatePickerStyleProvider>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// RangeCalendar
////////////////////////////////////////////////////////////////////////////////
export interface RangeDatePickerProps<T extends DateValue>
    extends AriaRangeCalendarProps<T>,
        VariantProps<typeof datePickerStyles> {
    ref?: RefObject<HTMLDivElement>
    /**
     * Renders multiple months at one time.
     * @default 1
     */
    visibleMonths?: 1 | 2 | 3
}

export const RangeDatePicker = <T extends DateValue>(props: RangeDatePickerProps<T>) => {
    const { locale } = useLocale()

    const calendarRef = useObjectRef(props.ref)

    // We must memo this to prevent the useCalendarState call from
    // making unnecessary state changes
    const visibleMonths = props.visibleMonths ?? 1
    const visibleDuration = useMemo(() => ({ months: visibleMonths }), [visibleMonths])

    // TODO: Test performance characteristics of memoizing the creation
    //       of the object.
    const state = useRangeCalendarState({
        ...props,
        locale,
        createCalendar,
        visibleDuration,
    })

    const { calendarProps, errorMessageProps, nextButtonProps, prevButtonProps } = useRangeCalendar(
        props,
        state,
        calendarRef,
    )

    const style = useStyles(datePickerStyles, props)

    return (
        <DatePickerStyleProvider value={style}>
            <CalendarContainer
                calendarProps={calendarProps}
                calendarRef={calendarRef}
                errorMessageProps={errorMessageProps}
                nextButtonProps={nextButtonProps}
                prevButtonProps={prevButtonProps}
                state={state}
                visibleMonths={visibleMonths}
            />
        </DatePickerStyleProvider>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// CalendarContainer
////////////////////////////////////////////////////////////////////////////////
interface CalendarContainerProps {
    calendarProps: DOMAttributes<FocusableElement>
    calendarRef: RefObject<HTMLDivElement>
    errorMessageProps: DOMAttributes<FocusableElement>
    nextButtonProps: AriaButtonProps
    prevButtonProps: AriaButtonProps
    state: CalendarState | RangeCalendarState
    visibleMonths: 1 | 2 | 3
}

const CalendarContainer = (props: CalendarContainerProps) => {
    const { styles } = useDatePickerStyles()

    const dateFormatter = useDateFormatter({
        month: "long",
        year: "numeric",
    })

    const startDate = props.state.visibleRange.start

    const headers = []
    const calendars = []

    for (let i = 0; i < props.visibleMonths; i++) {
        const visibleDate = startDate.add({ months: i })

        headers.push(
            <div key={i} {...styles.headerItem()}>
                {i === 0 && (
                    <Button
                        {...mergeProps(props.prevButtonProps, styles.headerPrev())}
                        variant="light"
                    >
                        <ChevronLeft />
                    </Button>
                )}
                <h2 {...styles.headerText()}>
                    {dateFormatter.format(visibleDate.toDate(props.state.timeZone))}
                </h2>
                {i === props.visibleMonths - 1 && (
                    <Button
                        {...mergeProps(props.nextButtonProps, styles.headerNext())}
                        variant="light"
                    >
                        <ChevronRight />
                    </Button>
                )}
            </div>,
        )

        calendars.push(<CalendarMonth {...props} key={i} startDate={visibleDate} />)
    }

    const start = new CalendarDate(2023, 12, 1)

    return (
        <div {...mergeProps(props.calendarProps, styles.wrapper())} ref={props.calendarRef}>
            <div {...styles.headerGroup()}>{headers}</div>
            <div {...styles.calendarGroup()}>{calendars}</div>
        </div>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// CalendarMonth
////////////////////////////////////////////////////////////////////////////////
interface CalendarMonthProps extends AriaCalendarGridProps {
    state: CalendarState | RangeCalendarState
}

const CalendarMonth = (props: CalendarMonthProps) => {
    const { styles } = useDatePickerStyles()

    const { gridProps, headerProps, weekDays } = useCalendarGrid(props, props.state)

    const { locale } = useLocale()
    // TODO: FIX
    // @ts-expect-error
    const weeksInMonth = getWeeksInMonth(props.startDate, locale)

    return (
        <table {...mergeProps(gridProps, styles.calendarItem())}>
            <thead {...headerProps}>
                <tr>
                    {weekDays.map((day, index) => (
                        <th {...styles.calendarCell()} key={index}>
                            <span {...styles.calendarDayOfWeek()}>{day}</span>
                        </th>
                    ))}
                </tr>
            </thead>
            <tbody>
                {[...new Array(weeksInMonth).keys()].map((weekIndex) => (
                    <tr key={weekIndex}>
                        {props.state.getDatesInWeek(weekIndex, props.startDate).map((date, i) =>
                            date ? (
                                <CalendarCell
                                    date={date}
                                    state={props.state}
                                    // TODO: FIX
                                    // @ts-expect-error
                                    targetMonth={props.startDate}
                                />
                            ) : (
                                <td key={i} />
                            ),
                        )}
                    </tr>
                ))}
            </tbody>
        </table>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// CalendarCell
////////////////////////////////////////////////////////////////////////////////
interface CalendarCellProps extends AriaCalendarCellProps {
    state: CalendarState | RangeCalendarState
    targetMonth: CalendarDate
}

const CalendarCell = (props: CalendarCellProps) => {
    const { styles } = useDatePickerStyles()

    const cellRef = useRef<HTMLElement>(null)

    const {
        buttonProps,
        cellProps,
        formattedDate,
        isDisabled,
        isFocused,
        isInvalid,
        isPressed,
        isSelected,
    } = useCalendarCell(props, props.state, cellRef)

    const { focusProps, isFocusVisible } = useFocusRing()
    const { hoverProps, isHovered } = useHover(props)

    const { locale } = useLocale()

    const highlightedRange = "highlightedRange" in props.state && props.state.highlightedRange
    const dayOfWeek = getDayOfWeek(props.date, locale)
    const isLastSelectedBeforeDisabled =
        !isDisabled && !isInvalid && props.state.isCellUnavailable(props.date.add({ days: 1 }))
    const isFirstSelectedAfterDisabled =
        !isDisabled && !isInvalid && props.state.isCellUnavailable(props.date.subtract({ days: 1 }))
    const isRangeStart =
        isSelected && (isFirstSelectedAfterDisabled || dayOfWeek === 0 || props.date.day === 1)
    const isRangeEnd =
        isSelected &&
        (isLastSelectedBeforeDisabled ||
            dayOfWeek === 6 ||
            props.date.day === props.targetMonth.calendar.getDaysInMonth(props.targetMonth))

    const isSelectionStart =
        isSelected && highlightedRange && isSameDay(props.date, highlightedRange.start)
    const isSelectionEnd =
        isSelected && highlightedRange && isSameDay(props.date, highlightedRange.end)

    const isOutsideMonth = !isSameMonth(props.date, props.targetMonth)

    return (
        <td {...mergeProps(cellProps, styles.calendarCell())}>
            <span
                {...mergeProps(
                    buttonProps,
                    focusProps,
                    hoverProps,
                    styles.calendarDay(
                        isSelected && styles.calendarDay.isRange,
                        isRangeStart && styles.calendarDay.isRangeStart,
                        isRangeEnd && styles.calendarDay.isRangeEnd,
                        isSelectionStart && styles.calendarDay.isSelectionStart,
                        isSelectionEnd && styles.calendarDay.isSelectionEnd,
                        isOutsideMonth && styles.calendarDay.outsideMonth,
                    ),
                )}
                ref={cellRef}
                // TODO:
                // style={{
                //     visibility: isOutsideMonth ? "hidden" : undefined,
                // }}
                data-focused={isFocused || undefined}
                data-focus-visible={isFocusVisible || undefined}
                data-hovered={isHovered || undefined}
                data-selection-start={isSelectionStart || undefined}
                data-selection-range={(isSelected && !!highlightedRange) || undefined}
                data-selection-end={isSelectionEnd || undefined}
                data-range-start={isRangeStart || undefined}
                data-range-end={isRangeEnd || undefined}
            >
                <span {...styles.calendarDayInner()}>
                    <span>{formattedDate}</span>
                </span>
            </span>
        </td>
    )
}
