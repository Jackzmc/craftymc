<template>
<div>
  <NavBar :has-sidebar="hasSidebar" @sidebar="hasSidebar = !hasSidebar" @update-modpacks="updateModpacks" />
  <br>
  <div class="columns mt-6" v-if="settings">
    <div class="column is-2 ml-1" v-show="hasSidebar">
      <SideBar />
    </div>
    <div class="column mr-1 ml-1">
      <router-view :settings="settings" :modpacks="modpacks" @update-settings="updateSettings"  />
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref, onBeforeMount } from 'vue'
import NavBar from '@/components/NavBar.vue'
import SideBar from '@/components/SideBar.vue'
import { invoke } from '@tauri-apps/api/tauri'
import { AppSettings } from './types/Settings';

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

onBeforeMount(async() => {
  await updateSettings()
  console.debug('app settings', settings.value)
  await updateModpacks()
  console.debug(modpacks.value.length, 'modpacks loaded')
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
