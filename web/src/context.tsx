import { createContext } from "react";
import * as wasm from "ldraw-renderer"

// separate file for hmr
export const RenderingContext = createContext((() => {
    const rendering = new wasm.RenderingNever();
    const proxy = rendering.get_proxy();
    try {
        rendering.run()
    } catch (error) {
        console.error(error)
    }
    return proxy
})());