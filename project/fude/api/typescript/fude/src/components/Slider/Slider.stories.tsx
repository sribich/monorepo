import { Label, SliderOutput } from "react-aria-components"
import { stylesToArgTypes } from "../../theme/props"
import { Slider, SliderThumb, SliderTrack } from "./Slider"
import { sliderStyles } from "./Slider.stylex"

const meta = {
    title: "Components/Slider/Primitive/Slider",
    component: Slider,
    tags: ["autodocs"],
    argTypes: stylesToArgTypes(sliderStyles),
} satisfies Meta<typeof Slider>

export default meta
type Story = StoryObj<typeof Slider>

export const Default = (props) => <Slider label="Amount" {...props}></Slider>

export const Vertical = () => <Slider></Slider>

export const WithChildren = () => <Slider className="foo" maxValue={360}></Slider>

// export const WithRange = () => <RangeSlider label="Test"></RangeSlider>
