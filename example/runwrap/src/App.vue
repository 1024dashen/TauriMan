<template>
    <div class="container" id="app">
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
                    <el-button class="inputBtn" @click="selectDir('outputDir')">
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
                <!-- tool -->
                <div class="toolBox">
                    <!-- 搜索 -->
                    <el-input
                        v-model="searchValue"
                        class="searchInput"
                        :placeholder="t('placeholder')"
                        @input="search"
                    />
                    <el-select
                        v-if="selectedRows.length > 0"
                        v-model="plan"
                        placeholder=""
                        :disabled="selectedRows.length === 0"
                        size="small"
                        class="selectBox"
                        @change="batchUpdatePolicy"
                    >
                        <el-option
                            v-for="item in planOptions"
                            :key="item.value"
                            :label="item.label"
                            :value="item.value"
                        />
                    </el-select>
                    <!-- 删除选中 -->
                    <el-button
                        v-if="selectedRows.length > 0"
                        :disabled="selectedRows.length === 0"
                        class="inputBtn"
                        @click="batchDelete"
                        type="danger"
                        plain
                    >
                        {{ t('deleteSelected') }}
                    </el-button>
                </div>
            </div>
            <div class="headerRight">
                <el-button
                    @click="runBundleCmd"
                    :disabled="btnDisabled || selectedRows.length === 0"
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
                    row-key="index"
                    :empty-text="t('empty')"
                    :scrollbar-always-on="false"
                    @selection-change="batchSelect"
                >
                    <el-table-column type="selection" width="30" />
                    <el-table-column
                        prop="index"
                        :label="t('index')"
                        width="60"
                        align="center"
                    />
                    <el-table-column prop="name" sortable :label="t('name')" />
                    <el-table-column
                        prop="size"
                        :label="t('size')"
                        width="110"
                        sortable
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
                        :label="t('policy')"
                        width="80"
                        align="center"
                    >
                        <template #default="scope">
                            <el-select
                                v-model="scope.row.policy"
                                placeholder=""
                                size="small"
                                class="selectBox"
                            >
                                <el-option
                                    v-for="item in planOptions"
                                    :key="item.value"
                                    :label="item.label"
                                    :value="item.value"
                                />
                            </el-select>
                        </template>
                    </el-table-column>
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
                                v-if="
                                    scope.row.state === 1 ||
                                    scope.row.state === 2
                                "
                                @click="openLogFile(scope.row.name)"
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

    <!-- 全局loading -->
    <el-dialog
        v-model="transLoading"
        fullscreen
        :show-close="false"
        class="buildDialog"
    >
        <div class="transLoading">
            <div class="loadingContainer">
                <div class="elLoadingSpinner">
                    <svg class="circular" viewBox="25 25 50 50">
                        <circle
                            class="path"
                            cx="50"
                            cy="50"
                            r="20"
                            fill="none"
                        ></circle>
                    </svg>
                </div>
            </div>
            <div class="loadingText">
                {{ t('transFileing') }} {{ currentFile }}...
            </div>
            <el-button
                class="stopBtn"
                :disabled="bundleState"
                @click="stopTrans"
                size="default"
                type="danger"
                plain
            >
                {{ t('stopTrans') }}
            </el-button>
        </div>
    </el-dialog>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { timeFormat } from './utils/common'
import { useI18n } from 'vue-i18n'
import VConsole from 'vconsole'
import { join } from '@tauri-apps/api/path'
import { exists, mkdir, writeTextFile } from '@tauri-apps/plugin-fs'
import i18n from '@/lang'
import { ElMessage, ElMessageBox } from 'element-plus'
import { load } from '@tauri-apps/plugin-store'

// 转换loading
const transLoading = ref(false)
// 国际化
const { t } = useI18n()
// exe dir
const exeDir = ref('')
let store: any = null

// 输入文件夹
const inputDir = ref(localStorage.getItem('inputDir') || '')
// 输出文件夹
const outputDir = ref(localStorage.getItem('outputDir') || '')
// 按钮状态是否可用
const btnDisabled = ref(false)

// 原始表格数据
const sourceData = ref<any[]>([
    {
        index: 1,
        name: '第一个001aaaBBBB.prt',
        size: '100KB',
        update: '2021-01-01 12:00:00',
        state: 0,
        policy: 0,
    },
    {
        index: 2,
        name: '第二季002ddddAdd.stp',
        size: '100KB',
        update: '2021-01-01 12:00:00',
        state: 0,
        policy: 0,
    },
    {
        index: 3,
        name: '第三季003jjjjjDDDD.x_t',
        size: '100KB',
        update: '2021-01-01 12:00:00',
        state: 0,
        policy: 0,
    },
    {
        index: 4,
        name: '第四季004addd.x_t',
        size: '100KB',
        update: '2021-01-01 12:00:00',
        state: 0,
        policy: 0,
    },
])

// 搜索后的表格数据
const tableData = ref<any[]>(sourceData.value)

// 日志
const transLog = ref<any[]>([])

// 加工策略
const plan = ref(0)
const planOptions = ref<any[]>([
    {
        value: 0,
        label: '钢',
    },
    {
        value: 1,
        label: '铝',
    },
    {
        value: 2,
        label: '铜',
    },
])

// 批量选中操作
const selectedRows = ref<any[]>([])
const batchSelect = (rows: any[]) => {
    console.log('批量选中操作', rows)
    selectedRows.value = rows
}

// 批量修改加工策略
const batchUpdatePolicy = () => {
    console.log('批量修改加工策略', selectedRows.value)
    selectedRows.value.forEach((item) => {
        item.policy = plan.value
    })
}

// 批量删除选中
const batchDelete = () => {
    console.log('批量删除选中', selectedRows.value)
    ElMessageBox.confirm('确定要删除选中文件吗？', 'Warning', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        center: true,
        type: 'warning',
    })
        .then(() => {
            tableData.value = tableData.value.filter(
                (item) => !selectedRows.value.includes(item)
            )
        })
        .catch(() => {
            console.log('取消删除')
        })
}

// stopTrans
const stopTrans = () => {
    console.log('停止转换')
    bundleState.value = true
}

// 模糊搜索过滤表格数据
const searchValue = ref('')
const search = (value: string) => {
    console.log('模糊搜索', value)
    if (value === '') {
        tableData.value = sourceData.value
    } else {
        tableData.value = sourceData.value.filter((item) =>
            item.name.toLowerCase().includes(value.toLowerCase())
        )
    }
}

// 选择文件夹
const selectDir = async (type: string) => {
    open({
        directory: true,
    }).then(async (res) => {
        console.log('选择文件夹', res)
        if (res === null || res === undefined) {
            return
        }
        if (type === 'inputDir') {
            inputDir.value = res || ''
            // localStorage.setItem('inputDir', inputDir.value)
            store && (await store.set('inputDir', { value: inputDir.value }))
            res && readDir(res)
        } else {
            outputDir.value = res || ''
            // localStorage.setItem('outputDir', outputDir.value)
            store && (await store.set('outputDir', { value: outputDir.value }))
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
    sourceData.value = res
        .map((item: any) => ({
            ...item,
            index: index++,
            state: 0,
            policy: plan.value,
            update: timeFormat(item.modified),
        }))
        .filter(
            (item: any) =>
                item.name.endsWith('.prt') ||
                item.name.endsWith('.stp') ||
                item.name.endsWith('.x_t')
        )
    // 表格数据
    tableData.value = sourceData.value
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
const runCommand = async (command: string, fileName?: string) => {
    try {
        // const rockcamrun = await join(currentDir, 'config', 'bin', 'fnm')
        const rockcamrun = await join(exeDir.value, 'rockcamrun')
        console.log('rockcamrun------', rockcamrun)
        const result: string = await invoke('run_command', {
            command: `${rockcamrun} ${command}`,
        })
        console.log('run_command------', result)
        fileName && (await writeLog(fileName, result))
        if (
            (result &&
                (result.includes('copied') || result.includes('--help'))) ||
            result.includes('schemes')
        ) {
            return result
        } else {
            return false
        }
    } catch (error) {
        console.error('执行命令失败', error)
        fileName && (await writeLog(fileName, JSON.stringify(error)))
        return false
    }
}

// 写入日志
const writeLog = async (
    fileName: string,
    log: string,
    append: boolean = false
) => {
    // 创建logs文件夹
    const logsDir = await join(exeDir.value, 'logs')
    const exist = await exists(logsDir)
    if (!exist) {
        await mkdir(logsDir)
    }
    const logPath = await join(logsDir, `${fileName}.txt`)
    await writeTextFile(logPath, log, { append })
}

// 用记事本打开log文件
const openLogFile = async (fileName: string) => {
    const logPath = await join(exeDir.value, 'logs', `${fileName}.txt`)
    console.log('打开日志文件', logPath)
    await invoke('run_command', {
        command: `Start-Process -FilePath "${logPath}"`,
    })
}

// 执行转换文件逻辑
const currentFile = ref<any>('')
const transFile = async (file: any, isBundle: boolean = false) => {
    const fileName = file.name
    const filePolicy = file.policy
    console.log('执行命令', fileName)
    if (!inputDir.value || !outputDir.value) {
        ElMessage.error('请先选择输入和输出文件夹')
        return
    }
    transLoading.value = true
    btnDisabled.value = true
    // 记录日志:开始时间，结束时间，文件名，状态，持续时间
    let startTime = new Date().getTime()
    let logString = ''
    logString += `开始时间: ${startTime.toLocaleString()}，`
    try {
        const inputFilePath = await join(inputDir.value, fileName)
        console.log(
            'inputFilePath',
            inputFilePath,
            'outputDir',
            outputDir.value
        )
        currentFile.value = fileName
        const result = await runCommand(
            `rockconvert -i "${inputFilePath}" -o "${outputDir.value}" -s ${filePolicy}`,
            fileName
        )
        if (result) {
            // 记录成功信息
            logString += `文件 ${fileName} 转换成功，`
            file.state = 1
        } else {
            // 记录错误信息
            logString += `文件 ${fileName} 转换失败，`
            file.state = 2
        }
    } catch (error) {
        // 记录错误信息
        logString += `文件 ${fileName} 转换失败，失败原因：${error}，`
        console.error('执行命令失败', error)
        ElMessage.error(`执行命令失败: ${error}`)
        file.state = 2
    } finally {
        if (isBundle) {
            console.log('批量执行...')
        } else {
            transLoading.value = false
        }
        let endTime = new Date().getTime()
        // 持续时间是多少分多少秒
        logString += `结束时间: ${endTime.toLocaleString()}，`
        logString += `持续时间: ${Math.floor(
            (endTime - startTime) / 60000
        )}分${Math.floor(((endTime - startTime) % 60000) / 1000)}秒\r`
        transLog.value.push(logString)
        btnDisabled.value = false
        console.log('执行命令完成', transLog.value)
    }
}

// 批量执行是否要停止
const bundleState = ref(false)
// 批量执行命令
const runBundleCmd = async () => {
    console.log('批量执行命令')
    if (!inputDir.value || !outputDir.value) {
        ElMessage.error('请先选择输入和输出文件夹')
        return
    }
    transLoading.value = true
    // 转换选中的文件
    for (const item of selectedRows.value) {
        if (bundleState.value) {
            console.log('批量执行命令停止')
            bundleState.value = false
            transLoading.value = false
            return
        } else {
            await transFile(item, true)
        }
    }
    transLoading.value = false
    ElMessage.success('批量转换完成')
}

// 移除某个数据
const deleteRow = (index: number) => {
    tableData.value.splice(index, 1)
}

// 初始化语言
const initLang = async () => {
    try {
        const manStr: string = await invoke('get_man')
        const manJson = JSON.parse(manStr)
        console.log('manJson invoke---', manJson)
        const UGII_LANG: string = await invoke('get_env_var', {
            name: 'UGII_LANG',
        })
        const localLang = UGII_LANG || 'english'
        console.log('localLang---', localLang)
        i18n.global.setLocaleMessage(localLang, manJson.langs[localLang])
        i18n.global.locale.value = localLang
    } catch (error) {
        console.error('获取man失败', error)
    }
}

// 禁用右键
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

// 获取环境变量
const getEnvVar = async () => {
    console.log('getEnvVar------')
    const envVar = await invoke('get_env_var', {
        name: 'UGII_LANG',
    })
    console.log('envVar---', envVar)
    const osInfo = await invoke('get_os_info')
    console.log('osInfo---', osInfo)
}

// run help
const initEnv = async () => {
    store = await load('store.json', { autoSave: true })
    // 获取exe文件夹
    exeDir.value = await invoke('get_exe_dir')
    // 检查是否存在rockcamrun
    const check = await runCommand('--help', 'programe-init')
    if (check) {
        console.log('执行帮助命令成功')
    } else {
        console.error('执行帮助命令失败')
        ElMessage.error('没有检测到 rockcamrun 程序，请检查安装')
        btnDisabled.value = true
    }
    // 初始化策略
    const policyStr = await runCommand('sysschemes')
    if (policyStr) {
        console.log('获取策略成功')
        try {
            const policy = JSON.parse(policyStr)
            console.log('policy---', policy)
            planOptions.value = policy.schemes.map(
                (item: any, index: number) => ({
                    label: item,
                    value: index,
                })
            )
            if (
                policy.selection &&
                policy.selection < planOptions.value.length
            ) {
                plan.value = policy.selection
            } else {
                // 默认选择第一个策略
                plan.value = 0
            }
            plan.value = policy.selection
        } catch (error) {
            console.error('获取策略失败', error)
            ElMessage.error('获取策略失败')
            btnDisabled.value = true
        }
    } else {
        console.error('获取策略失败')
        ElMessage.error('没有检测到策略，请检查安装')
        btnDisabled.value = true
    }
    // 初始化输入输出目录
    inputDir.value = ((await store.get('inputDir')) || { value: '' }).value
    outputDir.value = ((await store.get('outputDir')) || { value: '' }).value
    console.log('inputDir', inputDir.value, 'outputDir', outputDir.value)
    inputDir.value && readDir(inputDir.value)
}

onMounted(async () => {
    console.log('mounted------', inputDir.value, outputDir.value)
    await initLang()
    await initEnv()
    await getEnvVar()
    // 禁用右键
    !import.meta.env.DEV && disableRightClick()
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
        height: 7rem;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        padding: 1.25rem 1.25rem 0 1.25rem;
        gap: 1.25rem;

        .headerLeft {
            display: flex;
            flex-direction: column;
            gap: 0.625rem;
            flex: 1;

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

            .toolBox {
                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: flex-start;

                .searchInput {
                    width: 20rem;
                }

                .searchBtn {
                    margin: 0 0.625rem;
                }

                .toolTitle {
                    width: 90px;
                    font-size: 0.75rem;
                    color: #4f5154;
                }

                .selectBox {
                    max-width: 6rem;
                    margin: 0 0.625rem;
                }
            }
        }

        .headerRight {
            width: 18rem;
            height: 100%;
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            gap: 0.625rem;

            .batchBtn {
                width: 100%;
                height: 90%;
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
            min-width: 20rem;
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
            width: 18rem;
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

.transLoading {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 80vh;

    .loadingContainer {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .elLoadingSpinner {
        position: relative;
        width: 50px;
        height: 50px;
    }

    .loadingText {
        margin-top: 1.25rem;
        font-size: 1.25rem;
        color: #409eff;
    }

    .stopBtn {
        margin-top: 1.25rem;
    }

    .elLoadingSpinner .circular {
        width: 100%;
        height: 100%;
        animation: loading-rotate 2s linear infinite;
    }

    .elLoadingSpinner .path {
        animation: loading-dash 1.5s ease-in-out infinite;
        stroke-dasharray: 90, 150;
        stroke-dashoffset: 0;
        stroke-width: 2;
        stroke: #409eff;
        stroke-linecap: round;
    }

    @keyframes loading-rotate {
        100% {
            transform: rotate(360deg);
        }
    }

    @keyframes loading-dash {
        0% {
            stroke-dasharray: 1, 200;
            stroke-dashoffset: 0;
        }
        50% {
            stroke-dasharray: 90, 150;
            stroke-dashoffset: -40px;
        }
        100% {
            stroke-dasharray: 90, 150;
            stroke-dashoffset: -120px;
        }
    }
}
</style>
