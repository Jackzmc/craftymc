<template>
<div>
  <AskTelemetryModal v-if="settings && settings.general.telemetryState < 0" @result="(level) => settings.general.telemetryState = level" />
  <NavBar :has-sidebar="hasSidebar" @sidebar="hasSidebar = !hasSidebar" @update-modpacks="updateModpacks" />
  <br>
  <div class="columns mt-6" v-if="settings">
    <div class="column is-2 ml-1" v-show="hasSidebar">
      <SideBar />
    </div>
    <div :class="mainViewClass">
      <router-view :settings="settings" :modpacks="modpacks" @update-settings="updateSettings"  />
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref, onBeforeMount, computed } from 'vue'
import NavBar from '@/components/NavBar.vue'
import SideBar from '@/components/SideBar.vue'
import { invoke, convertFileSrc } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { AppSettings } from './types/Settings';
import { documentDir, join } from '@tauri-apps/api/path'
import AskTelemetryModal from '@/components/modals/AskTelemetryModal.vue'
import DefaultPackImage from '@/assets/default_pack.png'

const hasSidebar = ref(true)
let settings = ref<AppSettings>()
let modpacks = ref<Modpack[]>([])

async function updateSettings(newSettings?: AppSettings) {
  if(!newSettings) settings.value = await invoke('plugin:config|get_settings')
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
    newModpack.imageUrl = await _get_img_url(pack)
    modpacks.value.push(newModpack)
  }
}
async function _get_img_url(modpack: Modpack): Promise<Modpack> {
  return (modpack.img_ext)
    ? await convertFileSrc(await join(await documentDir(), `MCModDownloader/Instances/${modpack.folder_name}/pack.${modpack.img_ext}`))
    : DefaultPackImage
}

const mainViewClass = computed(() => {
  const arr = ['column', 'mr-1 ml-1']
  if(hasSidebar.value) arr.push("is-10")
  else arr.push("mx-5")
  return arr
})

onBeforeMount(async() => {
  await updateSettings()
  console.debug('app settings', settings.value)
  await updateModpacks()
  console.debug(modpacks.value.length, 'modpacks loaded')

  await listen('update-modpack', async(event) => {
    console.debug('update-modpack', event.payload.modpack)
    const newModpack = event.payload.modpack
    newModpack.imageUrl = await _get_img_url(newModpack)
    for(let i = 0; i < modpacks.value.length; i++) {
      if(modpacks.value[i].id === newModpack.id) {
        if(event.payload.deleted)
          modpacks.slice(i, 1)
        else
          modpacks.value[i] = newModpack
        return
      }
    }
    // Modpack to update not found, insert
    modpacks.value.push(newModpack)
  })
})

</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}
</style>
