<template>
<div>
  <div class="field">
    <input class="is-checkradio" id="flag1" type="checkbox" :value="TelemetryFlags.GeneralSystem" v-model="flags">
    <label for="flag1">General Information</label>
    <p class="help">Collects: OS Type (windows, linux, macos) and version (win10, win11, etc), and the country your ip resides in</p>
  </div>
  <div class="field">
    <input class="is-checkradio" id="flag2" type="checkbox" :value="TelemetryFlags.OnError" v-model="flags">
    <label for="flag2">Auto send crash reports or errors that occur</label>
  </div>
  <div class="field">
    <input class="is-checkradio" id="flag3" type="checkbox" :value="TelemetryFlags.Usage" v-model="flags">
    <label for="flag3">Send usage data</label>
    <p class="help">Such as a count of what modloaders and mod sources are used.</p>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const emit = defineEmits(["change"])

enum TelemetryFlags {
  None = 0,
  GeneralSystem = 1,
  OnError = 2,
  Usage = 4
}

let flags = ref([TelemetryFlags.GeneralSystem, TelemetryFlags.Usage])
watch(flags, () => {
  emit('change', flags.value.reduce((pv, cv) => pv + cv, 0))
})
</script>
