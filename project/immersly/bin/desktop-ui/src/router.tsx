import { createRootRoute, createRoute } from "@tanstack/react-router"
import { DictionaryLookupPage } from "./features/dictionary/pages/DictionaryLookupPage"
import { DictionarySettingsPage } from "./features/dictionary/pages/DictionarySettingsPage"
import { LibraryPage } from "./features/library/pages/LibraryPage"
import { ReadBookPage } from "./features/reader/pages/ReadBookPage"
import { SettingsPage } from "./features/settings/pages/SettingsPage"
import { StudyPage } from "./features/study/pages/StudyPage"
import { RootRoute } from "./routes/__root"
import { AppLayout } from "./routes/_app"
import { LibraryEditPage } from "./features/library/pages/LibraryEditPage"

const rootRoute = createRootRoute({
    component: RootRoute,
    notFoundComponent: () => {
        return <div>404</div>
    },
})

const rootLayout = createRoute({
    getParentRoute: () => rootRoute,
    id: "rootLayout",
    component: AppLayout,
})

const studyRoute = createRoute({
    getParentRoute: () => rootLayout,
    path: "study",
    component: StudyPage,
})

const dictionaryRoute = createRoute({
    getParentRoute: () => rootLayout,
    path: "dictionary",
})

const dictionaryIndexRoute = createRoute({
    getParentRoute: () => dictionaryRoute,
    path: "/",
    component: DictionaryLookupPage,
})

export const dictionarySettingsRoute = createRoute({
    getParentRoute: () => dictionaryRoute,
    path: "settings",
    component: DictionarySettingsPage,
})

const libraryRoute = createRoute({
    getParentRoute: () => rootLayout,
    path: "library",
})

const libraryIndexRoute = createRoute({
    getParentRoute: () => libraryRoute,
    path: "/",
    component: LibraryPage,
})

export const libraryEditRoute = createRoute({
    getParentRoute: () => libraryRoute,
    path: "$book_id/edit",
    component: () => {
        const { book_id: bookId } = libraryEditRoute.useParams()

        return <LibraryEditPage bookId={bookId} />
    },
})

export const libraryReadRoute = createRoute({
    getParentRoute: () => libraryRoute,
    path: "$book_id/read",
    component: () => {
        const { book_id: bookId } = libraryReadRoute.useParams()

        return <ReadBookPage bookId={bookId} />
    },
})

export const settingsRoute = createRoute({
    getParentRoute: () => rootLayout,
    path: "settings",
    component: SettingsPage,
})

/*

const DeactivatedImmerseRoute = createRoute({
  id: '/_deactivated/immerse',
  path: '/immerse',
  getParentRoute: () => rootRouteImport,
})
const DeactivatedAnkiRoute = createRoute({
  id: '/_deactivated/anki',
  path: '/anki',
  getParentRoute: () => rootRouteImport,
})
 */

export const routeTree = rootRoute.addChildren([
    rootLayout.addChildren([
        studyRoute,
        dictionaryRoute.addChildren([dictionaryIndexRoute, dictionarySettingsRoute]),
        libraryRoute.addChildren([libraryIndexRoute, libraryEditRoute, libraryReadRoute]),
        settingsRoute,
    ]),
])
