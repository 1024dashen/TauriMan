// index.ts
import { invoke } from '@tauri-apps/api/core'
import { createI18n } from 'vue-i18n'

let manJson: any = {
    name: 'TauriMan',
    version: '0.1.0',
    description: 'TauriMan is a tool for managing TauriMan',
    author: 'TauriMan',
    license: 'MIT',
    locale: 'en',
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
            check: 'Check',
            index: 'Index',
            name: 'Name',
            size: 'Size',
            update: 'Update',
            batch: 'Batch',
            status: 'Status',
            success: 'Success',
            waiting: 'Waiting',
            delete: 'Delete',
            record: 'Record',
            action: 'Action',
            empty: 'No data',
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
            check: '查看',
            index: '序号',
            name: '文件名',
            size: '文件体积(k)',
            update: '最近修改时间',
            batch: '批量执行',
            status: '状态',
            success: '成功',
            waiting: '等待',
            delete: '删除',
            record: '记录',
            action: '执行',
            empty: '暂无数据',
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
    const manStr: string = await invoke('get_man')
    manJson = JSON.parse(manStr)
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
