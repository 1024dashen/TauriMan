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
                    <el-button class="inputBtn" @click="selectDir('inputDir')">
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
                    <el-button class="inputBtn" @click="selectDir('inputDir')">
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
                <el-button
                    @click="runBundleCmd"
                    :disabled="btnDisabled"
                    class="batchBtn"
                >
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
                        sortable
                    />
                    <el-table-column prop="name" sortable :label="t('name')" />
                    <el-table-column
                        prop="size"
                        :label="t('size')"
                        sortable
                        width="100"
                    />
                    <el-table-column
                        prop="update"
                        :label="t('update')"
                        width="140"
                        align="center"
                        sortable
                    />
                    <el-table-column
                        prop="action"
                        :label="t('action')"
                        width="80"
                        align="center"
                    >
                        <template #default="scope">
                            <el-button
                                :disabled="btnDisabled"
                                @click="transFile(scope.row)"
                            >
                                <el-icon
                                    class="actionIcon"
                                    :class="{ disabled: btnDisabled }"
                                    size="20"
                                >
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
                            <!-- // state 0: 未转换 1: 转换成功 2: 转换失败 -->
                            <span
                                :class="{
                                    success: scope.row.state === 1,
                                    faile: scope.row.state === 2,
                                }"
                                >{{
                                    scope.row.state === 0
                                        ? t('waiting')
                                        : scope.row.state === 1
                                        ? t('success')
                                        : t('faile')
                                }}</span
                            >
                        </template>
                    </el-table-column>
                    <el-table-column
                        prop="record"
                        :label="t('record')"
                        width="80"
                        align="center"
                    >
                        <template #default="scope">
                            <el-button
                                :disabled="btnDisabled"
                                v-if="scope.row.state === 1"
                                @click="openDir(outputDir)"
                            >
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
                            <el-button
                                :disabled="btnDisabled"
                                type="danger"
                                plain
                                @click.prevent="deleteRow(scope.$index)"
                            >
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
                    <span>{{ item }}</span>
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
import { join } from '@tauri-apps/api/path'
import { ElMessage } from 'element-plus'
import { exists, writeTextFile } from '@tauri-apps/plugin-fs'
import i18n from '@/lang'

// 转换loading
const transLoading = ref(false)
// 国际化
const { t } = useI18n()
// exe dir
const exeDir = ref('')

// 输入文件夹
const inputDir = ref(localStorage.getItem('inputDir') || '')
// 输出文件夹
const outputDir = ref(localStorage.getItem('outputDir') || '')
// 按钮状态是否可用
const btnDisabled = ref(false)

// 表格数据
const tableData = ref<any[]>([])
// 日志
const transLog = ref<any[]>([])

// 选择文件夹
const selectDir = (type: string) => {
    open({
        directory: true,
    }).then((res) => {
        console.log('选择文件夹', res)
        if (type === 'inputDir') {
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
    // state 0: 未转换 1: 转换成功 2: 转换失败, 只获取文件名后缀为.prt .stp .x_t的文件
    tableData.value = res
        .map((item: any) => ({
            ...item,
            index: index++,
            state: 0,
            update: timeFormat(item.modified),
        }))
        .filter(
            (item: any) =>
                item.name.endsWith('.prt') ||
                item.name.endsWith('.stp') ||
                item.name.endsWith('.x_t')
        )
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

// 执行命令
const runCommand = async (command: string) => {
    try {
        // const rockcamrun = await join(currentDir, 'config', 'bin', 'fnm')
        const rockcamrun = await join(
            exeDir.value,
            'config',
            'bin',
            'rockcamrun'
        )
        console.log('rockcamrun------', rockcamrun)
        const result: string = await invoke('run_command', {
            command: `${rockcamrun} ${command}`,
        })
        console.log('run_command------', result)
        if (
            result &&
            (result.includes('copied') || result.includes('--help'))
        ) {
            return true
        } else {
            return false
        }
    } catch (error) {
        console.error('执行命令失败', error)
        return false
    }
}

// run help
const initEnv = async () => {
    // 获取exe文件夹
    exeDir.value = await invoke('get_exe_dir')
    // 检查是否存在rockcamrun
    const check = await runCommand('--help')
    if (check) {
        console.log('执行帮助命令成功')
    } else {
        console.error('执行帮助命令失败')
        ElMessage.error('没有检测到 rockcamrun 程序，请检查安装')
        btnDisabled.value = true
    }
    // 检查log是否存在
    const logPath = await join(exeDir.value, 'log.txt')
    const logExists = await exists(logPath)
    if (logExists) {
        console.log('存在')
    } else {
        await writeTextFile(logPath, 'init log')
    }
}

// 执行转换文件逻辑
const transFile = async (file: any, isBundle: boolean = false) => {
    const fileName = file.name
    console.log('执行命令', fileName)
    if (!inputDir.value || !outputDir.value) {
        ElMessage.error('请先选择输入和输出文件夹')
        return
    }
    transLoading.value = true
    btnDisabled.value = true
    try {
        const inputFilePath = await join(inputDir.value, fileName)
        console.log(
            'inputFilePath',
            inputFilePath,
            'outputDir',
            outputDir.value
        )
        loadingText(`正在转换文件 ${fileName}...`)
        const result = await runCommand(
            `-i "${inputFilePath}" -o ${outputDir.value}`
        )
        if (result) {
            // 记录成功信息
            const logString = `文件 ${fileName} 转换成功`
            transLog.value.push(logString)
            file.state = 1
        } else {
            // 记录错误信息
            const logString = `文件 ${fileName} 转换失败`
            transLog.value.push(logString)
            file.state = 2
        }
    } catch (error) {
        // 记录错误信息
        const logString = `文件 ${fileName} 转换失败`
        transLog.value.push(logString)
        console.error('执行命令失败', error)
        ElMessage.error(`执行命令失败: ${error}`)
        file.state = 2
    } finally {
        if (isBundle) {
            console.log('批量执行...')
        } else {
            transLoading.value = false
        }
        btnDisabled.value = false
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
        await transFile(item, true)
    }
    transLoading.value = false
    ElMessage.success('批量转换完成')
}

// 移除某个数据
const deleteRow = (index: number) => {
    tableData.value.splice(index, 1)
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

const disableRightClick = () => {
    //禁止F12
    document.onkeydown = function (event: any) {
        var winEvent: any = window.event
        if (winEvent && winEvent.keyCode == 123) {
            event.keyCode = 0
            event.returnValue = false
        }
        if (winEvent && winEvent.keyCode == 13) {
            winEvent.keyCode = 505
        }
    }
    //屏蔽右键菜单
    document.oncontextmenu = function (event: any) {
        if (window.event) {
            event = window.event
        }
        try {
            var the = event.srcElement
            if (
                !(
                    (the.tagName == 'INPUT' &&
                        the.type.toLowerCase() == 'text') ||
                    the.tagName == 'TEXTAREA'
                )
            ) {
                return false
            }
            return true
        } catch (e) {
            return false
        }
    }
}

onMounted(async () => {
    console.log('mounted')
    await initLang()
    await initEnv()
    !import.meta.env.DEV && disableRightClick()
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

                .disabled {
                    color: #c0c4cc !important;
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

.success {
    color: #67c23a;
}

.faile {
    color: #f56c6c;
}
</style>
