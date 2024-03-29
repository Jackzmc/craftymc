<template>
<div class="container">
  <h4 class="title is-4">Settings</h4>
  <div class="columns">
    <div class="column is-8">
      <Tabs inner-wrapper-class="tabs" :options="{useUrlFragment:false}">
        <Tab name="General">
          <Field label="Telemetry">
            <TelemetryList @change="setTelemetry" />
          </Field>
        </Tab>
        <Tab name="Minecraft">
          <h4 class="title is-4">Minecraft</h4>
          <!--<Field label="Save Directory">
            <input class="input" type="text" v-model="settings.minecraft.saveDirectory" />
          </Field>
          TODO: Remove or use this
          -->
          <Field label="Preferred Release">
            <div class="select">
              <select v-model="settings.minecraft.preferredRelease">
                 <option v-for="(display, release) in RELEASES" :key="release" :value="release">{{display}}</option>
              </select>
            </div>
          </Field>
          <hr>
          <h4 class="title is-4">Game Resolution</h4>
          <div class="columns">
            <div class="column is-4">
              <HorizontalField label="Width">
                <input class="input" type="text" v-model="settings.minecraft.width" />
              </HorizontalField>
            </div>
            <div class="column is-4">
              <HorizontalField label="Height">
                <input class="input" type="text" v-model="settings.minecraft.height" />
              </HorizontalField>
            </div>
          </div>
          <hr>
          <h4 class="title is-4">Java Settings</h4>
          <input id="javamemslider" class="slider is-fullwidth has-output" step="250" min="1000" :max="settings.meta.maxMemoryMb" v-model.number="settings.minecraft.javaMemoryMb" type="range" :data-tooltip="javaMemory">
          <Field label="Java Version">
            <div class="select">
              <select v-model="settings.minecraft.javaPath">
                <option :value="null">Automatic</option>
              </select>
            </div>
          </Field>
          <Field label="Additional Java Arguments">
            <input class="input" type="text" v-model="settings.minecraft.javaArgs" />
          </Field>
          <!-- save folder -->
          <!-- preferred release -->
          <!-- resolution, width/height -->
          <!-- launcher settings -->
          <!-- java settings -->
        </Tab>
      </Tabs>
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { Tabs, Tab } from 'vue3-tabs-component'
import Field from '@/components/form/Field.vue'
import HorizontalField from '@/components/form/HorizontalField.vue'
import { ref, computed, watch } from 'vue'
import { AppSettings } from '../types/Settings';
import { invoke } from '@tauri-apps/api/tauri'
import TelemetryList from '@/components/TelemetryList.vue'
import { onBeforeRouteLeave } from 'vue-router'

const emit = defineEmits(["update-settings"])
const props = defineProps<{
  settings: AppSettings
}>()

const RELEASES = {
  release: "Stable (Recommended)",
  beta: "Beta",
  alpha: "Alpha"
}

let settings = ref<AppSettings>(JSON.parse(JSON.stringify(props.settings)))

watch(settings, findAnyChange, { deep: true, immediate: true })

async function findAnyChange() {
  for(const category in settings.value) {
    for(const key in settings.value[category]) {
      if(key === "telemetryState") continue //why is vue3 watchers so janky
      if(props.settings[category][key] !== settings.value[category][key]) {
        await invoke('set_setting', {
          category,
          key,
          value: settings.value[category][key].toString()
        })
      }
    }
  }
}

async function setTelemetry(value) {
  settings.value.general.telemetryState = value
  await invoke('set_setting', {
    category: "general",
    key: "telemetryState",
    value: value.toString()
  })
}

const javaMemory = computed(() => {
  return `${settings.value.minecraft.javaMemoryMb.toLocaleString()} MB`
})

onBeforeRouteLeave(async() => {
  await findAnyChange()
  await invoke('save_settings')
  emit('update-settings', settings.value)
})
</script>
