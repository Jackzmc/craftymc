<template>
<div>
  <NavBar :has-sidebar="hasSidebar" @sidebar="hasSidebar = !hasSidebar" />
  <br>
  <div class="columns mt-6" v-if="settings">
    <div class="column is-2 ml-1" v-show="hasSidebar">
      <SideBar />
    </div>
    <div class="column mr-1 ml-1">
      <router-view :settings="settings" />
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

onBeforeMount(async() => {
  settings.value = await invoke('get_settings')
  console.log('app settings', settings.value)
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
