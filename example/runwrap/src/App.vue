<template>
    <div
        class="container"
        id="app"
        v-loading.fullscreen.lock="transLoading"
        element-loading-text="转换中"
    >
        <!-- header -->
        <div class="header">
            <div class="headerLeft">
                <div class="inputBox">
                    <el-button class="inputBtn" @click="selectDir('input')">
                        {{ t('inputDir') }}
                    </el-button>
                    <el-input
                        v-model="inputDir"
                        class="inputDir"
                        placeholder=""
                    />
                    <span class="checkBtn" @click="openDir(inputDir)">
                        {{ t('check') }}
                    </span>
                </div>
                <div class="inputBox">
                    <el-button class="inputBtn" @click="selectDir('output')">
                        {{ t('outputDir') }}
                    </el-button>
                    <el-input
                        v-model="outputDir"
                        class="inputDir"
                        placeholder=""
                    />
                    <span class="checkBtn" @click="openDir(outputDir)">
                        {{ t('check') }}
                    </span>
                </div>
            </div>
            <div class="headerRight">
                <el-button @click="runBundleCmd" class="batchBtn">
                    {{ t('start') }}
                </el-button>
            </div>
        </div>
        <!-- content -->
        <div class="content">
            <div class="contentLeft">
                <el-table
                    :data="tableData"
                    height="100%"
                    border
                    class="tableBox"
                    :empty-text="t('empty')"
                    :scrollbar-always-on="false"
                >
                    <el-table-column
                        prop="index"
                        :label="t('index')"
                        width="60"
                        align="center"
                    />
                    <el-table-column prop="name" :label="t('name')" />
                    <!-- <el-table-column
                        prop="size"
                        :label="t('size')"
                        width="100"
                    /> -->
                    <el-table-column
                        prop="update"
                        :label="t('update')"
                        width="140"
                        align="center"
                    />
                    <el-table-column
                        prop="action"
                        :label="t('action')"
                        width="80"
                        align="center"
                    >
                        <template #default="scope">
                            <el-button @click="runCommand(scope.row.name)">
                                <el-icon class="actionIcon" size="20">
                                    <CaretRight />
                                </el-icon>
                            </el-button>
                        </template>
                    </el-table-column>
                    <el-table-column
                        prop="state"
                        :label="t('status')"
                        width="64"
                        align="center"
                    >
                        <template #default="scope">
                            {{ t('waiting') }}
                        </template>
                    </el-table-column>
                    <el-table-column
                        prop="record"
                        :label="t('record')"
                        width="80"
                        align="center"
                    >
                        <template #default="scope">
                            <el-button @click="runHelp">
                                <el-icon><Document /></el-icon>
                            </el-button>
                        </template>
                    </el-table-column>
                    <el-table-column
                        prop="delete"
                        :label="t('delete')"
                        width="80"
                        align="center"
                    >
                        <template #default="scope">
                            <el-button type="danger" plain>
                                <el-icon><Delete /></el-icon>
                            </el-button>
                        </template>
                    </el-table-column>
                </el-table>
            </div>
            <el-scrollbar class="contentRight" height="100%">
                <div
                    v-for="(item, index) in transLog"
                    :key="index"
                    class="logItem"
                >
                    <span>文件名: {{ item.fileName }}</span>
                    <span>开始时间: {{ item.startTime }}</span>
                    <span>结束时间: {{ item.endTime }}</span>
                    <span>持续时间: {{ item.during }}</span>
                    <span>转换状态: {{ item.success }}</span>
                </div>
            </el-scrollbar>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { timeFormat, loadingText } from './utils/common'
import { useI18n } from 'vue-i18n'
import VConsole from 'vconsole'
import { Command } from '@tauri-apps/plugin-shell'
import { join } from '@tauri-apps/api/path'
import { ElMessage } from 'element-plus'
import i18n from '@/lang'

const transLoading = ref(false)

const { t } = useI18n()

const inputDir = ref(localStorage.getItem('inputDir') || '')
const outputDir = ref(localStorage.getItem('outputDir') || '')

const tableData = ref<any[]>([])

const transLog = ref<any[]>([])

// 选择文件夹
const selectDir = (type: string) => {
    open({
        directory: true,
    }).then((res) => {
        console.log('选择文件夹', res)
        if (type === 'input') {
            inputDir.value = res || ''
            localStorage.setItem('inputDir', inputDir.value)
            res && readDir(res)
        } else {
            outputDir.value = res || ''
            localStorage.setItem('outputDir', outputDir.value)
        }
    })
}

// 读取文件夹
const readDir = async (dir: string) => {
    console.log('读取文件夹', dir)
    const res: any = await invoke('read_dir', {
        path: dir,
    })
    console.log('读取文件夹结果', res)
    let index = 1
    tableData.value = res.map((item: any) => ({
        ...item,
        index: index++,
        update: timeFormat(item.modified),
    }))
}

// 打开文件夹
const openDir = (dir: string) => {
    console.log('打开文件夹', dir)
    if (!dir || dir === '') {
        ElMessage.error('请先选择输入和输出文件夹')
        return
    }
    if (dir === 'debug') {
        const _ = new VConsole()
    } else {
        invoke('open_folder', {
            path: dir,
        })
    }
}

// run help
const runHelp = async () => {
    try {
        const command = await invoke('run_command', {
            command:
                'D:\\ShenProject\\TauriMan\\src-tauri\\config\\bin\\fnm.exe --version',
        })
        console.log('run_command------', command)
    } catch (error) {
        console.error('执行命令失败', error)
    }
}

// 执行命令
const runCommand = async (fileName: string, isBundle: boolean = false) => {
    console.log('执行命令', fileName)
    // 创建本次执行的记录
    const logString: any = {
        fileName,
        startTime: new Date().toISOString(),
        inputDir: inputDir.value,
        outputDir: outputDir.value,
    }

    if (!inputDir.value || !outputDir.value) {
        ElMessage.error('请先选择输入和输出文件夹')
        return
    }
    transLoading.value = true
    try {
        const inputFilePath = await join(inputDir.value, fileName)
        console.log(
            'inputFilePath',
            inputFilePath,
            'outputDir',
            outputDir.value
        )
        const command = Command.sidecar('bin/rockcamrun', [
            '-i',
            inputFilePath,
            '-o',
            outputDir.value,
        ])
        loadingText(`正在转换文件 ${fileName}...`)
        const output = await command.execute()
        console.log('command output', output)
        console.log('out:', output.stdout)
        console.log('err:', output.stderr)
        // 记录成功信息
        logString.endTime = new Date().toISOString()
        logString.success = true
        logString.durationMs =
            new Date(logString.endTime).getTime() -
            new Date(logString.startTime).getTime()
        transLog.value.push(logString)
    } catch (error) {
        // 记录错误信息
        logString.endTime = new Date().toISOString()
        logString.success = false
        logString.error = error instanceof Error ? error.message : String(error)
        logString.durationMs =
            new Date(logString.endTime).getTime() -
            new Date(logString.startTime).getTime()
        transLog.value.push(logString)
        console.error('执行命令失败', error)
        ElMessage.error(`执行命令失败: ${error}`)
    } finally {
        if (isBundle) {
            console.log('批量执行...')
        } else {
            transLoading.value = false
        }
        console.log('执行命令完成', transLog.value)
    }
}

// 批量执行命令
const runBundleCmd = async () => {
    console.log('批量执行命令')
    if (!inputDir.value || !outputDir.value) {
        ElMessage.error('请先选择输入和输出文件夹')
        return
    }
    transLoading.value = true
    for (const item of tableData.value) {
        await runCommand(item.name, true)
    }
    transLoading.value = false
    ElMessage.success('批量转换完成')
}

const initLang = async () => {
    try {
        const manStr: string = await invoke('get_man')
        const manJson = JSON.parse(manStr)
        console.log('manJson invoke---', manJson)
        i18n.global.setLocaleMessage(
            manJson.locale,
            manJson.langs[manJson.locale]
        )
        i18n.global.locale.value = manJson.locale
    } catch (error) {
        console.error('获取man失败', error)
    }
}

onMounted(() => {
    console.log('mounted')
    initLang()
    inputDir.value && readDir(inputDir.value)
})
</script>

<style scoped lang="scss">
.container {
    width: 100%;
    height: 100%;
    background-color: #f0f2f5;
    display: flex;
    flex-direction: column;
    overflow: hidden;

    .header {
        height: 3.125rem;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        padding: 1.25rem;
        gap: 1.25rem;

        .headerLeft {
            display: flex;
            flex-direction: column;
            gap: 0.625rem;
            flex: 1;

            .inputBtn {
                width: 10rem;
            }

            .inputBox {
                display: flex;
                flex-direction: row;
                align-items: center;
                gap: 0.625rem;

                .inputDir {
                    flex: 1;
                }

                .checkBtn {
                    color: #409eff;
                    cursor: pointer;
                    font-size: 0.75rem;
                    // text-decoration: underline;
                }
            }
        }

        .headerRight {
            width: 18.75rem;
            height: 100%;
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            gap: 0.625rem;

            .batchBtn {
                width: 100%;
                height: 100%;
            }
        }
    }

    .content {
        flex: 1;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        padding: 0 1.25rem 1.25rem 1.25rem;
        gap: 1.25rem;
        overflow: hidden;

        .contentLeft {
            flex: 1;
            width: 100%;
            overflow-y: hidden;

            .tableBox {
                overflow: hidden;
                // overflow-y: scroll;

                .actionIcon {
                    color: #409eff;
                }

                .deleted {
                    color: #f56c6c;
                }
            }
        }

        .contentRight {
            width: 18.75rem;
            border: 0.0625rem solid rgb(223, 223, 223);
            border-radius: 0.625rem;
            // overflow-y: scroll;

            .logItem {
                color: gray;
                margin: 0.625rem;
                font-size: 0.75rem;
            }
        }
    }
}
</style>
