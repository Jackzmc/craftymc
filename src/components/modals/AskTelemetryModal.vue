<template>
  <BaseModal active title="Welcome!" show-header>
    <p>
      Thank you for using this software. To help development, we ask you to enable telemetry information to get valuable information to help with usage knowledge and issues.
      This is a completely anonymous and opt-in experience.</p>
    <p>You can change this option at any time in the <em>[Settings]</em> menu on the left</p>
    <br>
    <p>Select any options you wish to enable or leave blank to disallow any telemetry:</p>
    <br>
    <TelemetryList @change="setLevel" />
    <template v-slot:footer>
      <div class="buttons">
        <a class="button is-success" @click="enable">
          Save
        </a>
      </div>
    </template>
  </BaseModal>
</template>

<script setup lang="ts">
import BaseModal from './BaseModal.vue'
import { invoke } from '@tauri-apps/api/tauri'
import TelemetryList from '@/components/TelemetryList.vue'
import { ref } from 'vue'
const emit = defineEmits(['result'])

let level = ref(0)


async function enable() {
  await invoke("set_setting", {
    category: "general",
    key: "telemetryState",
    value: level.value.toString()
  })
  await invoke('save_settings')
  emit('result', level.value)
}

function setLevel(value) {
  level.value = value
}

</script>
