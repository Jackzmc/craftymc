<template>
<div>
  <component v-if="modal?.component" :is="modal.component" :pack="modal.pack" active
    @result="(level) => settings.general.telemetryState = level"
    @close="modal = undefined"
  />
  <NavBar :has-sidebar="hasSidebar" @sidebar="hasSidebar = !hasSidebar" @update-modpacks="updateModpacks" @installModloader="installModloader" />
  <br>
  <div class="columns mt-6 is-gapless" style="overflow-y: auto; overflow-x: hidden;" v-if="settings">
    <div class="column is-2" v-show="hasSidebar">
      <SideBar :show-data="showData" @selected="v => showResponseData = v" />
    </div>
    <router-view :class="mainViewClass" :settings="settings" :modpacks="modpacks" :invalid-modpacks="invalidPacks" :selected="showResponseData"
      @update-settings="updateSettings" @change-modloader="installModloader" @show="onShow"  />
  </div>
</div>
</template>

<script setup lang="ts">
import { ref, onBeforeMount, onMounted, computed, defineAsyncComponent, markRaw } from 'vue'
import NavBar from '@/components/NavBar.vue'
import SideBar from '@/components/SideBar.vue'

import { invoke, convertFileSrc } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { documentDir, join } from '@tauri-apps/api/path'

import { AppSettings } from './types/Settings';
import DefaultPackImage from '@/assets/default_pack.png'

const hasSidebar = ref(true)
let settings = ref<AppSettings>()
let modpacks = ref<Modpack[]>([])
let invalidPacks = ref<Partial<Modpack>[]>([])
let modal = ref<{ component: any, pack: any}>()
let showData = ref()
let showResponseData = ref()

async function updateSettings(newSettings?: AppSettings) {
  if(!newSettings) settings.value = await invoke('get_settings')
  else settings.value = newSettings
}

async function updateModpacks(newModpack?: Modpack) {
  if(!newModpack) {
    const packs = await invoke('get_modpacks')
    for(let pack of packs) {
      pack.imageUrl = await _get_img_url(pack)
    }
    modpacks.value = packs
  }
  else {
    newModpack.imageUrl = await _get_img_url(newModpack)
    modpacks.value.push(newModpack)
  }
}
async function _get_img_url(modpack: Modpack): Promise<Modpack> {
  return (modpack.img_ext)
    ? await convertFileSrc(await join(await documentDir(), `MCModDownloader/Instances/${modpack.folder_name}/pack.${modpack.img_ext}`))
    : DefaultPackImage
}

const mainViewClass = computed(() => {
  const arr = ['column']
  if(hasSidebar.value) arr.push("is-10")
  else arr.push("mx-5")
  return arr
})

enum UpdateModpackState {
  Normal = "Normal",
  Deleted = "Deleted",
  NowActive = "NowActive",
  Invalid = "Invalid"
}

onBeforeMount(async() => {
  await updateSettings()
  console.debug('app settings', Object.assign({}, settings.value))

  let isLauncherActive = false

  await listen('update-modpack', async(event) => {
    console.debug('update-modpack', event.payload)
    if(event.payload.state.Invalid) {
      return invalidPacks.value.push({ name: event.payload.state.Invalid[0], reason: event.payload.state.Invalid[1] })
    } else if(isLauncherActive && event.payload.state === UpdateModpackState.Normal) {
      modal.value = undefined
      isLauncherActive = false
      console.debug(`Launcher exited with code ${event.payload.data === null ? '<signal>' : event.payload.data}`)
      return
    } else if(event.payload.state.Importing) return

    const newModpack = event.payload.modpack
    if(newModpack) {
      newModpack.imageUrl = await _get_img_url(newModpack)

      // Find the modpack by it's id
      for(let i = 0; i < modpacks.value.length; i++) {
        if(modpacks.value[i].id === newModpack.id) {
          if(event.payload.state == UpdateModpackState.Deleted)
            modpacks.value.splice(i, 1)
          else if(event.payload.state === UpdateModpackState.NowActive) {
            modal.value = { component: markRaw(defineAsyncComponent(()  => import('@/components/modals/NowPlayingModal.vue'))), pack: event.payload.modpack }
            isLauncherActive = true
            return
          } else
            modpacks.value[i] = newModpack
          return
        }
      }
    }
    // If no known modpack, insert it
    modpacks.value.push(newModpack)
  })

  await updateModpacks()
  console.debug(`${modpacks.value.length} modpacks loaded`)
})

onMounted(() => {
  if(settings.value && settings.value.general.telemetryState < 0) {
    modal.value = { component: markRaw(defineAsyncComponent(()  => import('@/components/modals/AskTelemetryModal.vue'))), pack: null }
  }
})

function installModloader(pack: Modpack) {
  console.debug('installing modloader', pack)
  modal.value = { component: markRaw(defineAsyncComponent(() => import('@/components/modals/ModloaderInstaller.vue'))), pack: pack}
}
function onShow(value) {
  showData.value = value
}

</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}
html, body {
  /* overflow: hidden !important; */
}
</style>
