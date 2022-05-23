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
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { AppSettings } from './types/Settings';
import AskTelemetryModal from '@/components/modals/AskTelemetryModal.vue'

const hasSidebar = ref(true)
let settings = ref<AppSettings>()
let modpacks = ref<Modpack[]>([])

async function updateSettings(newSettings?: AppSettings) {
  if(!newSettings) settings.value = await invoke('get_settings')
  else settings.value = newSettings
}

async function updateModpacks(newModpack?: Modpack) {
  if(!newModpack) modpacks.value = await invoke('get_modpacks')
  else modpacks.value.push(newModpack)
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

  await listen('update-modpack', (event) => {
    console.debug('update-modpack', event.payload.modpack)
    const newModpack = event.payload.modpack
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
