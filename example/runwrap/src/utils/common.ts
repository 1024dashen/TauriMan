import { join } from "@tauri-apps/api/path"
import { readDir } from "@tauri-apps/plugin-fs"

// read dir recursively
export const readDirRecursively = async (path: string): Promise<string[]> => {
    const entries = await readDir(path)
    const fileList: string[] = []
    for (const entry of entries) {
        const fullPath = await join(path, entry.name)
        // 过滤隐藏文件
        if (entry.name.startsWith('.')) {
            continue
        }
        if (entry.isDirectory) {
            const subDirFiles = await readDirRecursively(fullPath)
            fileList.push(...subDirFiles)
        } else {
            fileList.push(fullPath)
        }
    }
    return fileList
}