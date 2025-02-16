import { compileAll, scanDir } from "./lib";
import { watchFile } from 'node:fs';



const throttle = (func: () => Promise<void>, limit: number) => {
    let inThrottle: NodeJS.Timeout | boolean;
    return () => {
        if (!inThrottle) {
            func()
            inThrottle = setTimeout(() => inThrottle = false, limit)
        }
    }
}

export async function watch() {
    await compileAll();
    console.log("Precompilation done, watching for changes...");
    const list = await scanDir("src");
    for (const file of list) {
        watchFile(file, throttle(async () => {
            await compileAll();
            console.log("Recompiled");
        }, 2000));
    }
}
void watch();