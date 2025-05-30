// index.ts
import { invoke } from '@tauri-apps/api/core'
import { createI18n } from 'vue-i18n'

let manJson: any = {
    name: 'TauriMan',
    version: '0.1.0',
    description: 'TauriMan is a tool for managing TauriMan',
    author: 'TauriMan',
    license: 'MIT',
    locale: 'zh-CN',
    langs: {
        en: {
            label: 'English',
            inputDir: 'Input Dir',
            outputDir: 'Output Dir',
            start: 'Start',
            stop: 'Stop',
            clear: 'Clear',
            about: 'About',
            help: 'Help',
        },
        'zh-CN': {
            label: '中文',
            inputDir: '输入文件夹',
            outputDir: '输出文件夹',
            start: '开始',
            stop: '停止',
            clear: '清除',
            about: '关于',
            help: '帮助',
        },
        'zh-TW': {
            label: '繁體中文',
            inputDir: '輸入文件夾',
            outputDir: '輸出文件夾',
            start: '開始',
            stop: '停止',
            clear: '清除',
            about: '關於',
            help: '幫助',
        },
    },
    window: {
        title: 'TauriMan',
        url: 'http://localhost:5173/',
        width: 960,
        height: 720,
        resizable: true,
        center: true,
        maximized: false,
    },
}

try {
    // const manStr: string = await invoke('get_man')
    // manJson = JSON.parse(manStr)
    console.log('manJson invoke---', manJson)
} catch (error) {
    console.error('获取man失败', error)
}

const i18n = createI18n({
    locale: manJson.locale || 'en',
    fallbackLocale: 'en',
    messages: manJson.langs,
    legacy: false,
    globalInjection: true,
})

export default i18n
