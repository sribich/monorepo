export class ExtendedSet<T> extends Set<T> {
    /**
     * Because a Set is not a functor, we cannot map over it.
     *
     * Instead of mapping between two sets, we must instead change
     * the containing type to an array.
     */
    public map<TNewType>(f: (value: T) => TNewType): TNewType[] {
        const newArray: TNewType[] = []

        for (const item of this) {
            newArray.push(f(item))
        }

        return newArray
    }

    public reduce<TNewType>(f: (acc: TNewType, value: T) => TNewType, initial: TNewType): TNewType {
        let result = initial

        for (const item of this) {
            result = f(result, item)
        }

        return result
    }

    public filter(f: (value: T) => boolean): ExtendedSet<T> {
        const newSet = new ExtendedSet<T>()

        for (const item of this) {
            if (f(item)) {
                newSet.add(item)
            }
        }

        return newSet
    }

    public every(f: (value: T) => boolean): boolean {
        for (const item of this) {
            if (!f(item)) {
                return false
            }
        }

        return true
    }

    public some(f: (value: T) => boolean): boolean {
        for (const item of this) {
            if (f(item)) {
                return true
            }
        }

        return false
    }
}
