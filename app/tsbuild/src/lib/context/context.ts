export abstract class Context {
    async initialise(): Promise<void> {}

    async terminate(): Promise<void> {}
}
