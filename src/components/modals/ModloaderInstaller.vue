<template>
<BaseModal active show-header :title="'Installing ' + props.pack.settings.modloaderType">

  <p>A window will open in 5 seconds, please select a version of your modloader to download.</p>
  <p>The window will automatically close once a download is completed.</p>

  <div class="has-text-centered my-5" v-if="waiting">Waiting for download</div>
  <template v-slot:footer>

  </template>
</BaseModal>
</template>

<script setup lang="ts">
import { onBeforeMount, ref } from 'vue'
import BaseModal from './BaseModal.vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen as appListen, emit as appEmit } from '@tauri-apps/api/event'

const emit = defineEmits(["close"])

const props = defineProps<{
  pack: Partial<Modpack>
}>()

let waiting = ref(false)
let waitForCloseTimer = ref<number>()

/*
watch_modloader_download -- download has started
  modloader_download_found -- jar found, prompt ui to close
  modloader_download_ready -- window has closed
  modloader_download_complete -- modloader acquired and moved
Ok(())
*/
onBeforeMount(async() => {
  invoke('watch_modloader_download', { // Do not await the promise, blocking
    packId: props.pack.id //props.pack.id, //FIXME: RESTORE THIS
  })
  setTimeout(() => {
    waiting.value = true
    let windowHandle
    if(props.pack.settings.modloaderType === "forge")
      windowHandle = window.open(`https://files.minecraftforge.net/net/minecraftforge/forge/index_${props.pack.versions.minecraft}.html`, '_tauri')
    else {
      alert("Unsupported modloader")
    }

    appListen('modloader_download_found', () => {
      windowHandle.close()
    })

    waitForCloseTimer.value = setInterval(() => {
      checkWindow(windowHandle)
    }, 1000)
  }, 1000 * 5)
})

async function checkWindow(windowHandle) {
  if(windowHandle.closed && waiting.value) {
    clearInterval(waitForCloseTimer.value)
    appEmit('modloader_download_ready')

    appListen("modloader_download_complete", () => {
      waiting.value = false
      emit('save')
    })
  }
}
</script>
