// export interface SelectFilterProps {
//     property: Immutable<PropertySchemaRepr<"select">>
//     filter: CheckboxFilterType & { property: string; uuid: string }
// }

export const SelectFilter = (/*props: SelectFilterProps*/) => {
    return null

    /*
    const schema = useSchema()
    const viewScope = useViewScopeContext()

    const { filter } = props
    
    let content = ""

    const property = getProperty(PropertyKind.Checkbox)
    const filterKeys = Object.keys(property.filter.filters).map((it) => ({ key: it }))

    const propertyName = schema.propertyEditor.getProperty(props.filter.property)?.name

    switch (filter.kind) {
        case "IS":
            content = filter.data ? "Checked" : "Unchecked"
            break
        case "IS_NOT":
            content = filter.data ? "Not Checked" : "Not Unchecked"
            break
        default:
            return void (filter satisfies never)
    }

    const onAction = (value: string | number) => {
        schema.viewEditor.updateFilter(viewScope.schema.uuid, props.filter.uuid, value === "Checked" ? true : false)
    }

    const onMoreAction = (action: string | number) => {
        switch (action) {
            case "delete":
                schema.viewEditor.deleteFilter(viewScope.schema.uuid, props.filter.uuid)
                return
            default: 
                throw new Error(`Unknown action ${action}`)
        }
    }

    return (
        <DialogTrigger>
            <DelegateButton>
                <Chip size="md" className="flex items-center">
                    {createElement(CheckboxProperty.icon, { size: 16, className: "mr-1" })}
                    <span>{propertyName}</span>
                    :
                    <span>{content}</span>
                    <ChevronDown size="16" />
                </Chip>
            </DelegateButton>
            <Popover>
                <Dialog>
                    <Box padding="2" rounded="md">
                        <Flex className="mb-2">
                            <Select className="flex-1" label={<TypographyText size="xs" color="secondary">Checkbox</TypographyText>} labelPlacement="side" size="xs" variant="light" defaultSelectedKey={filter.kind} items={filterKeys}>
                                {(item) => (
                                    <SelectItem id={item.key}>{item.key}</SelectItem>
                                )}
                            </Select>
                            <DialogTrigger>
                                <Button size="xs" variant="light" className="flex-0">
                                    <MoreHorizontal size="16" />
                                </Button>
                                <Popover>
                                    <Dialog>
                                        <Menu onAction={onMoreAction}>
                                            <MenuItem id="delete">Delete</MenuItem>
                                            <MenuItem id="convert">Convert to compound filter</MenuItem>
                                        </Menu>
                                    </Dialog>
                                </Popover>
                            </DialogTrigger>
                        </Flex>
                        <Menu onAction={onAction}>
                            <MenuItem id="Unchecked">Unchecked</MenuItem>
                            <MenuItem id="Checked">Checked</MenuItem>
                        </Menu>
                    </Box>
                </Dialog>
            </Popover>
        </DialogTrigger>
    )
    */
}
