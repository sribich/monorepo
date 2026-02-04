import { DatePicker, RangeDatePicker } from "../DatePicker"

const meta = {
    title: "Data Entry/DatePicker",
    component: DatePicker,
} satisfies Meta<typeof DatePicker>

export default meta

type Story = StoryObj<typeof meta>

export const Overview = (props) => <DatePicker visibleMonths={3} />

export const Range = (props) => <RangeDatePicker visibleMonths={3} />
