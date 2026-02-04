import type { Meta, StoryObj } from "@storybook/react-vite"
import { CardView, Card, CardHeader, CardFooter, CardBody } from "./Card"
import { GridLayout, ListLayout, WaterfallLayout } from "@react-stately/layout"
import { Size } from "@react-stately/virtualizer"
import { Virtualizer } from "react-aria-components"
import { Image } from "../Image/Image"
import preview from "@/preview"
import { stylesToArgTypes } from "../../theme/props.js"
import { cardStyles } from "./Card.stylex"
import { Divider } from "../Divider/Divider"

const meta = preview.meta({
    component: Card,
    argTypes: stylesToArgTypes(cardStyles),
    tags: ["autodocs"],
})

export const Overview = meta.story((props) => (
    <Card {...props}>
        <CardHeader>Header</CardHeader>
        <CardBody>Body</CardBody>
        <CardFooter>Footer</CardFooter>
    </Card>
))

export const WithDivider = meta.story((props) => (
    <Card {...props}>
        <CardHeader>Header</CardHeader>
        <Divider />
        <CardBody>Body</CardBody>
        <Divider />
        <CardFooter>Footer</CardFooter>
    </Card>
))

export const WithContent = meta.story((props) => (
    <Card {...props}>
        <CardHeader>...</CardHeader>
        <CardBody>
            <Image
                alt="Woman listing to music"
                style={{ objectFit: "cover", borderRadius: "12px" }}
                height={200}
                src="https://heroui.com/images/hero-card.jpeg"
                width={200}
            />
        </CardBody>
    </Card>
))

export const Virtualized = meta.story((props) => (
    <Virtualizer
        layout={WaterfallLayout}
        layoutOptions={{
            rowHeight: 32,
            minItemSize: new Size(32, 32),
            minSpace: new Size(32, 32),
            // minSpace: 0,
            padding: 550,
            gap: 550,
        }}
    >
        <CardView>
            <Card>Item 1</Card>
            <Card>Item 2</Card>
            <Card>Item 3</Card>
            <Card>Item 4</Card>
        </CardView>
    </Virtualizer>
))

export const Lone = meta.story((props) => (
    <Card>
        <Image height={200} src="https://heroui.com/images/hero-card.jpeg" width={200} />
        <CardFooter>Item 1</CardFooter>
    </Card>
))

/*
    <Card isFooterBlurred className="border-none" radius="lg">
      <Image
        alt="Woman listing to music"
        className="object-cover"
        height={200}
        src="https://heroui.com/images/hero-card.jpeg"
        width={200}
      />
      <CardFooter className="justify-between before:bg-white/10 border-white/20 border-1 overflow-hidden py-1 absolute before:rounded-xl rounded-large bottom-1 w-[calc(100%_-_8px)] shadow-small ml-1 z-10">
        <p className="text-tiny text-white/80">Available soon.</p>
        <Button
          className="text-tiny text-white bg-black/20"
          color="default"
          radius="lg"
          size="sm"
          variant="flat"
        >
          Notify me
        </Button>
      </CardFooter>
    </Card>
*/
