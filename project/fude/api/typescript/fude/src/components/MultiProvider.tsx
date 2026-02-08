import type { Context, ReactNode } from "react"

type ProviderValue<T> = [Context<T>["Provider"], T]
type ProviderValues<A, B, C, D, E, F, G, H, I, J, K> =
    | []
    | [ProviderValue<A>]
    | [ProviderValue<A>, ProviderValue<B>]
    | [ProviderValue<A>, ProviderValue<B>, ProviderValue<C>]
    | [ProviderValue<A>, ProviderValue<B>, ProviderValue<C>, ProviderValue<D>]
    | [ProviderValue<A>, ProviderValue<B>, ProviderValue<C>, ProviderValue<D>, ProviderValue<E>]
    | [
          ProviderValue<A>,
          ProviderValue<B>,
          ProviderValue<C>,
          ProviderValue<D>,
          ProviderValue<E>,
          ProviderValue<F>,
      ]
    | [
          ProviderValue<A>,
          ProviderValue<B>,
          ProviderValue<C>,
          ProviderValue<D>,
          ProviderValue<E>,
          ProviderValue<F>,
          ProviderValue<G>,
      ]
    | [
          ProviderValue<A>,
          ProviderValue<B>,
          ProviderValue<C>,
          ProviderValue<D>,
          ProviderValue<E>,
          ProviderValue<F>,
          ProviderValue<G>,
          ProviderValue<H>,
      ]
    | [
          ProviderValue<A>,
          ProviderValue<B>,
          ProviderValue<C>,
          ProviderValue<D>,
          ProviderValue<E>,
          ProviderValue<F>,
          ProviderValue<G>,
          ProviderValue<H>,
          ProviderValue<I>,
      ]
    | [
          ProviderValue<A>,
          ProviderValue<B>,
          ProviderValue<C>,
          ProviderValue<D>,
          ProviderValue<E>,
          ProviderValue<F>,
          ProviderValue<G>,
          ProviderValue<H>,
          ProviderValue<I>,
          ProviderValue<J>,
      ]
    | [
          ProviderValue<A>,
          ProviderValue<B>,
          ProviderValue<C>,
          ProviderValue<D>,
          ProviderValue<E>,
          ProviderValue<F>,
          ProviderValue<G>,
          ProviderValue<H>,
          ProviderValue<I>,
          ProviderValue<J>,
          ProviderValue<K>,
      ]

interface ProviderProps<A, B, C, D, E, F, G, H, I, J, K> {
    values: ProviderValues<A, B, C, D, E, F, G, H, I, J, K>
    children: ReactNode
}

export const MultiProvider = <A, B, C, D, E, F, G, H, I, J, K>(
    props: ProviderProps<A, B, C, D, E, F, G, H, I, J, K>,
): ReactNode => {
    let children = props.children

    for (const [Provider, value] of props.values) {
        children = <Provider value={value as never}>{children}</Provider>
    }

    return children
}
