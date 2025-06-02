import { join } from '@tauri-apps/api/path'
import { readDir } from '@tauri-apps/plugin-fs'

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

export const timeFormat = (timestamp: number) => {
    // 创建一个新的 Date 对象（注意：JS 时间戳是毫秒，所以需要 *1000）
    const date = new Date(timestamp * 1000)
    // 获取各个时间部分
    const year = date.getFullYear()
    const month = String(date.getMonth() + 1).padStart(2, '0') // 月份从0开始
    const day = String(date.getDate()).padStart(2, '0')
    const hours = String(date.getHours()).padStart(2, '0')
    const minutes = String(date.getMinutes()).padStart(2, '0')
    // 组合成目标格式
    return `${year}-${month}-${day} ${hours}:${minutes}`
}

// loading text
export const loadingText = (text: string) => {
    if (document.querySelector('.el-loading-text')) {
        document.querySelector('.el-loading-text')!.innerHTML = text
    } else {
        console.log('no loading')
    }
}
