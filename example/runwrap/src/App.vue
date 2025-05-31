<template>
    <div class="container">
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
                <el-button class="batchBtn">{{ t('start') }}</el-button>
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
                    scrollbar-always-on="false"
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
                            <el-button>
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
                            <el-button>
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
                    v-for="item in tableData"
                    :key="item.index"
                    class="logItem"
                >
                    这是日志数据，暂时是假数据，包含输入文件夹、输出文件夹、文件列表展示以及批量执行功能。
                </div>
            </el-scrollbar>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { timeFormat } from './utils/common'
import { useI18n } from 'vue-i18n'
import VConsole from 'vconsole'

const { t } = useI18n()

const inputDir = ref('')
const outputDir = ref('')

const tableData = ref<any[]>([])

// 选择文件夹
const selectDir = (type: string) => {
    open({
        directory: true,
    }).then((res) => {
        console.log('选择文件夹', res)
        if (type === 'input') {
            inputDir.value = res || ''
            res && readDir(res)
        } else {
            outputDir.value = res || ''
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
    if (dir === 'debug') {
        const _ = new VConsole()
    } else {
        invoke('open_folder', {
            path: dir,
        })
    }
}
onMounted(() => {
    console.log('mounted')
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
        height: 50px;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        padding: 20px;
        gap: 20px;

        .headerLeft {
            display: flex;
            flex-direction: column;
            gap: 10px;
            flex: 1;

            .inputBtn {
                width: 160px;
            }

            .inputBox {
                display: flex;
                flex-direction: row;
                align-items: center;
                gap: 10px;

                .inputDir {
                    flex: 1;
                }

                .checkBtn {
                    color: #409eff;
                    cursor: pointer;
                    // text-decoration: underline;
                }
            }
        }

        .headerRight {
            width: 300px;
            height: 100%;
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            gap: 10px;

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
        padding: 0 20px 20px 20px;
        gap: 20px;
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
            width: 300px;
            border: 1px solid rgb(223, 223, 223);
            border-radius: 10px;
            overflow-y: scroll;

            .logItem {
                color: gray;
                margin: 10px;
                font-size: 12px;
            }
        }
    }
}
</style>
