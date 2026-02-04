export abstract class SettingProvider<T> {
    public abstract getNamespace(): string
    public abstract getDefaults(): T
    public abstract getSettings(
        settings: unknown,
        containerEl: HTMLElement,
        save: () => Promise<void>,
    ): void
}
