<template>
<div class="container">
  <h4 class="title is-4">Settings</h4>
  <div class="columns">
    <div class="column is-8">
      <Tabs inner-wrapper-class="tabs" :options="{useUrlFragment:false}">
        <Tab name="General">

        </Tab>
        <Tab name="Minecraft">
          <h4 class="title is-4">Minecraft</h4>
          <Field label="Save Directory">
            <input class="input" type="text" v-model="settings.minecraft.saveDirectory" />
          </Field>
          <Field label="Preferred Release">
            <div class="select">
              <select>
                <option v-for="(display, release) in RELEASES" :key="release">{{display}}</option>
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
          <input id="javamemslider" class="slider is-fullwidth has-output" step="250" min="1000" max="8000" v-model.number="settings.minecraft.javaMemoryMb" type="range" :data-tooltip="javaMemory">
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
import { ref, computed } from 'vue'
import { AppSettings } from '../types/Settings';

const props = defineProps<{
  settings: AppSettings
}>()

// TODO: tauri invoke to get appmeta and settings

const RELEASES = {
  stable: "Stable (Recommended)",
  beta: "Beta",
  alpha: "Alpha"
}

let settings = ref<AppSettings>(props.settings)

const javaMemory = computed(() => {
  return `${settings.value.minecraft.javaMemoryMb.toLocaleString()} MB`
})
</script>
