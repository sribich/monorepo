import { Label, SliderOutput } from "react-aria-components"
import { stylesToArgTypes } from "../../theme/props"
import { Slider, SliderThumb, SliderTrack } from "./Slider"
import { sliderStyles } from "./Slider.stylex"
import preview from "@/preview"

const meta = preview.meta({
    title: "Components/Slider",
    component: Slider,
    tags: ["autodocs"],
    ...stylesToArgTypes(sliderStyles),
})

export const Overview = meta.story({
    render: (props) => <Slider label="Amount" {...props}></Slider>,
})

export const Vertical = meta.story({ render: () => <Slider></Slider> })

export const WithChildren = meta.story({
    render: () => <Slider className="foo" maxValue={360}></Slider>,
})

// export const WithRange = () => <RangeSlider label="Test"></RangeSlider>
