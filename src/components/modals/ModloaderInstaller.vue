<template>
<BaseModal active show-header title="Install modloader">
  <p class="subtitle is-4">Installing {{props.pack.settings.modloaderType}}</p>

  <p>A window will open in 5 seconds, please select a version of your modloader to download.</p>
  <p>Once the download completes, you may close that window</p>

  <div class="has-text-centered my-5" v-if="waiting">Waiting for download</div>
</BaseModal>
</template>

<script setup lang="ts">
import { onBeforeMount, ref } from 'vue'
import BaseModal from './BaseModal.vue'
import { invoke } from '@tauri-apps/api/tauri'

const props = defineProps<{
  pack: Partial<Modpack>
}>()

let waiting = ref(false)


onBeforeMount(async() => {
  await invoke('start_modloader_download', {
    packId: "_debug_ignore_" //props.pack.id, //FIXME: RESTORE THIS
  })
  setTimeout(() => {
    waiting.value = true
    let windowHandle
    if(props.pack.settings.modloaderType === "forge")
      windowHandle = window.open(`https://files.minecraftforge.net/net/minecraftforge/forge/index_${props.pack.versions.minecraft}.html`, '_tauri')
    else {
      alert("Unsupported modloader")
    }
    setInterval(() => {
      checkWindow(windowHandle)
    }, 1000)
  }, 1000 * 5)
})

function checkWindow(windowHandle) {
  if(windowHandle.closed) {
    waiting.value = false
    props.emit('save')
  }
}
</script>
