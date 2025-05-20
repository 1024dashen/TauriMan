<script setup lang="ts">
import { onMounted } from 'vue'
import {
    defaultWindowIcon,
    getName,
    getTauriVersion,
    getVersion,
    hide,
    setTheme,
    show,
} from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api/core'
import { Command } from '@tauri-apps/plugin-shell'

const init = async () => {
    // Write content to clipboard
    const version = await getVersion()
    console.log('version', version)
    const tauriVersion = await getTauriVersion()
    console.log('tauriVersion', tauriVersion)
    const name = await getName()
    console.log('name', name)
    const startDir = await invoke('get_exe_dir')
    console.log('startDir', startDir)
    // get man
    const manStr = await invoke('get_man')
    console.log('manStr---', manStr)
    const command = Command.sidecar('bin/fnm', ['--version'])
    const output = await command.execute()
    console.log('command output', output)
    console.log('out:', output.stdout)
    console.log('err:', output.stderr)
    // get machine uid
    const machineUid = await invoke('get_machine_uid')
    console.log('machineUid', machineUid)
}

onMounted(() => {
    init()
})
</script>

<template>
    <div>
        <a href="https://vite.dev" target="_blank">
            <img src="/vite.svg" class="logo" alt="Vite logo" />
        </a>
        <a href="https://vuejs.org/" target="_blank">
            <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
        </a>
    </div>
</template>

<style scoped>
.logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
}
.logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
    filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
