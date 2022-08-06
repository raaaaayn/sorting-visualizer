import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import wasm from "vite-plugin-wasm";

export default ({ mode }) => {
    return defineConfig({
        plugins: [react(), wasm()],
        define: {
            "process.env.NODE_ENV": `"${mode}"`,
        },
    });
};
