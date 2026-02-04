https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion

https://medium.com/eightshapes-llc/size-in-design-systems-64f234aec519

https://medium.com/eightshapes-llc/typography-in-design-systems-6ed771432f1e
https://www.youtube.com/watch?v=88XxC0_zs74

Components should only control what is inside of their "box". No margins outside.
Try to limit margins when components accept styling children, instead opt for flex control.

As a result, we’ve begun to approach unifying component size as a step-by-step process that includes typography, space, and height from the inside out.

Sizing

Determing your "checkpoints".

xs, sm, md, lg

xs - For content within other containers, such as buttons within an input, etc.
sm - Higher density smaller form displays. Toolbars, modals, etc.
md - Standard for content
lg - Bigger important items, typically the only choice in a form, etc.

## Identify components that require consistent sizing

Buttons/Inputs. Form control items should take up equal height unless wrapping is required. checkbox groups, etc.
Containing items. List boxes, etc. Unless long-form descriptions are used.

~24
~38
~46

# KEep fonts same between sizes

Use line height, keep it consistent ~1.2x
Letter spacing - for headers

## Consistent padding and margins
