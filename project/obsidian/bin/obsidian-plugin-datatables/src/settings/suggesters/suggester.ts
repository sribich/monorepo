import { type Instance, createPopper } from "@popperjs/core"
import { type ISuggestOwner } from "obsidian"

const wrapAround = (value: number, size: number): number => {
    return ((value % size) + size) % size
}

class Suggester<T> {
    private owner: ISuggestOwner<T>
    private values: T[] = []
    private suggestions: HTMLElement[] = []
    private selectedItem = 0
    private containerEl: HTMLElement

    constructor(owner: ISuggestOwner<T>, containerEl: HTMLElement) {
        this.owner = owner
        this.containerEl = containerEl

        containerEl.on("click", ".suggestion-item", this.onSuggestionClick.bind(this))
        containerEl.on("mousemove", ".suggestion-item", this.onSuggestionMouseover.bind(this))

        /*
        scope.register([], "ArrowUp", (event) => {
            if (!event.isComposing) {
                this.setSelectedItem(this.selectedItem - 1, true);
                return false;
            }
        });

        scope.register([], "ArrowDown", (event) => {
            if (!event.isComposing) {
                this.setSelectedItem(this.selectedItem + 1, true);
                return false;
            }
        });

        scope.register([], "Enter", (event) => {
            if (!event.isComposing) {
                this.useSelectedItem(event);
                return false;
            }
        });
        */
    }

    onSuggestionClick(event: MouseEvent, el: HTMLElement): void {
        event.preventDefault()

        const item = this.suggestions.indexOf(el)
        this.setSelectedItem(item, false)
        this.useSelectedItem(event)
    }

    onSuggestionMouseover(_event: MouseEvent, el: HTMLElement): void {
        const item = this.suggestions.indexOf(el)
        this.setSelectedItem(item, false)
    }

    setSuggestions(values: T[]) {
        this.containerEl.empty()
        const suggestionEls: HTMLDivElement[] = []

        values.forEach((value) => {
            const suggestionEl = this.containerEl.createDiv("suggestion-item")
            this.owner.renderSuggestion(value, suggestionEl)
            suggestionEls.push(suggestionEl)
        })

        this.values = values
        this.suggestions = suggestionEls
        this.setSelectedItem(0, false)
    }

    useSelectedItem(event: MouseEvent | KeyboardEvent) {
        const currentValue = this.values[this.selectedItem]
        if (currentValue) {
            this.owner.selectSuggestion(currentValue, event)
        }
    }

    setSelectedItem(selectedIndex: number, scrollIntoView: boolean) {
        const normalizedIndex = wrapAround(selectedIndex, this.suggestions.length)
        const prevSelectedSuggestion = this.suggestions[this.selectedItem]
        const selectedSuggestion = this.suggestions[normalizedIndex]

        prevSelectedSuggestion?.removeClass("is-selected")
        selectedSuggestion?.addClass("is-selected")

        this.selectedItem = normalizedIndex

        if (scrollIntoView) {
            selectedSuggestion?.scrollIntoView(false)
        }
    }
}

export abstract class InputSuggester<T> implements ISuggestOwner<T> {
    protected inputEl: HTMLInputElement | HTMLTextAreaElement

    private suggestEl: HTMLElement = createDiv("suggestion-container")

    private popper!: Instance
    private suggester: Suggester<T>

    constructor(inputEl: HTMLInputElement | HTMLTextAreaElement) {
        this.inputEl = inputEl

        const suggestion = this.suggestEl.createDiv("suggestion")
        this.suggester = new Suggester(this, suggestion)

        this.inputEl.addEventListener("input", this.onInputChanged.bind(this))
        this.inputEl.addEventListener("focus", this.onInputChanged.bind(this))

        this.inputEl.addEventListener("blur", this.close.bind(this))

        this.suggestEl.on("mousedown", ".suggestion-container", (event: MouseEvent) => {
            event.preventDefault()
        })
    }

    abstract getSuggestions(input: string): T[]

    abstract renderSuggestion(item: T, el: HTMLElement): void
    abstract selectSuggestion(item: T): void

    protected close(): void {
        // app.keymap.popScope(this.scope);

        this.suggester.setSuggestions([])

        this.popper?.destroy()

        this.suggestEl.detach()
    }

    private onInputChanged(): void {
        const input = this.inputEl.value
        const suggestions = this.getSuggestions(input)

        if (!suggestions || suggestions.length === 0) {
            return this.close()
        }

        this.suggester.setSuggestions(suggestions)
        this.open((app as any).dom.appContainerEl, this.inputEl)
    }

    private open(container: HTMLElement, inputEl: HTMLElement): void {
        // app.keymap.pushScope(this.scope);

        container.appendChild(this.suggestEl)

        this.popper = createPopper(inputEl, this.suggestEl, {
            placement: "bottom-start",
            modifiers: [
                {
                    name: "sameWidth",
                    enabled: true,
                    fn: ({ state, instance }) => {
                        // Note: positioning needs to be calculated twice -
                        // first pass - positioning it according to the width of the popper
                        // second pass - position it with the width bound to the reference element
                        // we need to early exit to avoid an infinite loop
                        const targetWidth = `${state.rects.reference.width}px`
                        if (state.styles["popper"]?.width === targetWidth) {
                            return
                        }

                        if (state.styles["popper"]) {
                            state.styles["popper"].width = targetWidth
                        }

                        instance.update()
                    },
                    phase: "beforeWrite",
                    requires: ["computeStyles"],
                },
            ],
        })
    }
}
