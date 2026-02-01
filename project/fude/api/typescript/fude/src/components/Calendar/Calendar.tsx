import { createCalendar, getWeeksInMonth } from "@internationalized/date"
import { type RefObject, useMemo, useRef } from "react"
import {
    type AriaCalendarCellProps,
    type AriaCalendarProps,
    type DateValue,
    useCalendar,
    useCalendarCell,
    useCalendarGrid,
    useFocusRing,
    useHover,
    useLocale,
} from "react-aria"
import { ButtonContext } from "react-aria-components"
import { type CalendarState, useCalendarState } from "react-stately"

import { useObjectRef } from "../../hooks/useObjectRef"
import { useStyles } from "../../theme/props"
import { MultiProvider } from "../../utils/context"
import { mergeProps } from "../../utils/mergeProps"
import { CalendarStyleProvider, calendarStyles, useCalendarStyles } from "./Calendar.stylex"

//==============================================================================
// Calendar
//==============================================================================
export namespace Calendar {
    export interface Props<T extends DateValue> extends AriaCalendarProps<T> {
        ref?: RefObject<HTMLDivElement>
    }
}

export const Calendar = <T extends DateValue>(props: Calendar.Props<T>) => {
    const ref = useObjectRef(props.ref)

    const { locale } = useLocale()
    const state = useCalendarState({
        ...props,
        createCalendar,
        locale,
        visibleDuration: { months: 1 },
    })
    const { calendarProps, nextButtonProps, prevButtonProps } = useCalendar(props, state)

    const { gridProps, headerProps, weekDays } = useCalendarGrid(
        {
            startDate: state.visibleRange.start,
            weekdayStyle: "short",
        },
        state,
    )

    const styles = useStyles(calendarStyles, {})

    const days = useMemo(() => {
        const weeksInMonth = getWeeksInMonth(state.visibleRange.start, locale)

        return [...new Array(weeksInMonth).keys()].map((weekIndex) => (
            <>
                {state
                    .getDatesInWeek(weekIndex, state.visibleRange.start)
                    .map((date) =>
                        date ? (
                            <CalendarDay
                                key={date.toDate("UTC").getTime()}
                                date={date}
                                state={state}
                            />
                        ) : (
                            <div key={date}></div>
                        ),
                    )}
            </>
        ))
    }, [state.visibleRange.start, locale])

    return (
        <MultiProvider
            values={[
                [
                    ButtonContext,
                    {
                        slots: {
                            nextMonth: nextButtonProps,
                            prevMonth: prevButtonProps,
                        },
                    },
                ],
                [CalendarStyleProvider, styles],
            ]}
        >
            <div {...mergeProps(calendarProps, gridProps, styles.styles.wrapper())} ref={ref}>
                <div {...mergeProps(headerProps, styles.styles.calendarHeader())}>
                    {weekDays.map((day) => (
                        <div key={day}>{day}</div>
                    ))}
                </div>
                <div {...mergeProps(styles.styles.calendarBody())}>{days}</div>
            </div>
        </MultiProvider>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// CalendarDay
////////////////////////////////////////////////////////////////////////////////
interface CalendarDayProps extends AriaCalendarCellProps {
    state: CalendarState
}

const CalendarDay = (props: CalendarDayProps) => {
    const { styles } = useCalendarStyles()

    const cellRef = useRef<HTMLElement>(null)
    const {
        cellProps,
        formattedDate,
        isDisabled,
        isFocused,
        isInvalid,
        isPressed,
        isSelected,
        isOutsideVisibleRange,
        isUnavailable,
    } = useCalendarCell(props, props.state, cellRef)

    const { focusProps, isFocusVisible } = useFocusRing()
    const { hoverProps, isHovered } = useHover(props)

    return (
        <div {...mergeProps(cellProps, focusProps, hoverProps, styles.day())}>{formattedDate}</div>
    )
}
