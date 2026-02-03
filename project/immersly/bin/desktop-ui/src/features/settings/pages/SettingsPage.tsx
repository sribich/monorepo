//==============================================================================
// SettingsPage
//==============================================================================
export namespace SettingsPage {
    export type Props = Record<string, never>
}

export const SettingsPage = (props: SettingsPage.Props) => {
    return null
}

/*
    <Tabs variant="underline">
            <Tab id="general" title="General">
                <ThemeToggle />
                <DataDirectory />
            </Tab>
            <Tab id="other" title="Other">
                <div>other</div>
            </Tab>
        </Tabs>


const ThemeToggle = () => {
    const theme = use(ThemeContext)

    return (
        <div>
            <Switch defaultSelected={theme.currentTheme === "dark"} onChange={(value) => theme.setTheme(value ? "dark" : "light")} />
        </div>
    )
}

const DataDirectory = () => {
    // const result = useMutation({
    //    mutationFn:
    // })

    const data = useQuery({
        queryKey: ["..."],
        queryFn: () => listSettings({}),
    })

    console.log(data.data)

    return (
        <div>
            <span>Data Directory</span>
            <span>/path/to/place</span>
            <Button>Change</Button>
        </div>
    )
}
*/
