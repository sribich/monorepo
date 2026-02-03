import react from "@vitejs/plugin-react"

/** @type {import('rolldown-vite').UserConfig} */
export default {
    plugins: [react()],
    server: {
        cors: {
            origin: false,
            preflightContinue: true,
        },
        proxy: {
            "/play": { target: "http://127.0.0.1:7057" },
            "/rpc": { target: "http://127.0.0.1:7057" },
        },
    },
}
