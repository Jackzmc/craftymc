<template>
<div class="container">
  <h4 class="title is-4">About {{ name }}</h4>
  <table class="table">
    <tr>
      <th>Name</th>
      <th>Value</th>
    </tr>
    <tr>
      <td>Tauri Version</td>
      <td>{{ tauriVersion }}</td>
    </tr>
    <tr>
      <td>Backend Version</td>
      <td>{{ version }}</td>
    </tr>
    <tr>
      <td>UI Version</td>
      <td>{{ UI_VERSION }}</td>
    </tr>
  </table>
</div>
</template>


<script setup lang="ts">
import { ref, onBeforeMount, inject } from 'vue'
import { getName, getTauriVersion, getVersion } from '@tauri-apps/api/app'

let name = ref()
let tauriVersion = ref()
let version = ref()

const UI_VERSION = inject("UI_VERSION")

onBeforeMount(async() => {
  name.value = await getName()
  tauriVersion.value = await getTauriVersion()
  version.value = await getVersion()
})

</script>
