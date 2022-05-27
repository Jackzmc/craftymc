<template>
<BaseModal active show-header :title="'Installing ' + props.pack.settings.modloaderType">

  <p>A window will open in 5 seconds, please select a version of your modloader's installer to download.</p>
  <p>The window will automatically close once a download is completed.</p>
  <br>
  <p>If the window does not open, <a :href="modloaderUrl">click here to open downloads page</a>. Place the installer in your Downloads folder.</p>

  <div class="has-text-centered my-5 subtitle is-4" v-if="waitingInstall">
    <fa-icon :icon="['fa', 'hourglass']" /><span> Installing modloader...</span>
  </div>
  <div class="has-text-centered my-5 subtitle is-4" v-else-if="waiting">
    <fa-icon :icon="['fa', 'hourglass']" /><span> Waiting for download...</span>
  </div>

  <template v-slot:footer>

  </template>
</BaseModal>
</template>

<script setup lang="ts">
import { onBeforeMount, ref, computed } from 'vue'
import BaseModal from './BaseModal.vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen as appListen, emit as appEmit } from '@tauri-apps/api/event'

const emit = defineEmits(["close", "save"])

const props = defineProps<{
  pack: Partial<Modpack>
}>()

let waiting = ref(false)
let waitingInstall = ref(false)
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
    if(modloaderUrl) {
      console.debug('Opening', modloaderUrl, 'in a new window')
      windowHandle = window.open(modloaderUrl, '_tauri')
    } else
      return alert("Unsupported modloader")

    appListen('modloader_download_found', () => {
      windowHandle.close()
    })

    waitForCloseTimer.value = setInterval(() => {
      checkWindow(windowHandle)
    }, 1000)
  }, 1000 * 5)
})

const modloaderUrl = computed(() => {
if(props.pack.settings.modloaderType === "forge")
    return `https://files.minecraftforge.net/net/minecraftforge/forge/index_${props.pack.versions.minecraft}.html`
  return undefined
})
async function checkWindow(windowHandle) {
  if(windowHandle.closed && waiting.value) {
    clearInterval(waitForCloseTimer.value)
    appEmit('modloader_download_ready')
    waitingInstall.value = true

    appListen("modloader_download_complete", () => {
      waiting.value = false
      emit('save')
    })
  }
}
</script>
