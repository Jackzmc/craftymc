<template>
<div class="container">
  <h4 class="title is-4">About {{ name }}</h4>
  <div class="columns">
    <div class="column">
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
    <div class="column is-4">
      <h6 class="title is-6">debug commands</h6>
      <ul>
        <li><a @click="invoke('plugin:debug|install_launcher')">debug_install_launcher</a></li>
        <li><a @click="invoke('plugin:debug|echo', {msg: 'hello!'})">debug_echo</a></li>
        </ul>
    </div>
  </div>
</div>
</template>


<script setup lang="ts">
import { ref, onBeforeMount, inject } from 'vue'
import { getName, getTauriVersion, getVersion } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api/tauri'

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
