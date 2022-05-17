<template>
<div class="container">
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
          <input id="javamemslider" class="slider is-fullwidth has-output" step="250" min="1000" max="8000" v-model.number="settings.minecraft.javaMemory" type="range" :data-tooltip="javaMemory">
          <Field label="Java Version">
            <div class="select">
              <select v-model="settings.minecraft.javaPath">
                <option :value="undefined">Automatic</option>
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
import Field from '@/components/Field.vue'
import HorizontalField from '@/components/HorizontalField.vue'
import { ref, computed } from 'vue'

interface AppSettings {
  general: GeneralSettings,
  minecraft: MinecraftSettings
}

// TODO: tauri invoke to get appmeta and settings
//eslint-disable-next-line
interface AppMeta {
  maxMemory: number
}

// Needs to be flat to tie to rust struct
interface GeneralSettings {

}
// Needs to be flat to tie to rust struct
interface MinecraftSettings {
  saveDirectory: string,
  width: number,
  height: number,
  javaMemory: number,
  javaPath: string,
  javaArgs: string
}


const RELEASES = {
  stable: "Stable (Recommended)",
  beta: "Beta",
  alpha: "Alpha"
}

const settings = ref<AppSettings>({
  general: {

  },
  minecraft: {
    javaMemory: 1000,
    javaPath: undefined,
    javaArgs: ""
  }
})

const javaMemory = computed(() => {
  return `${settings.value.minecraft.javaMemory.toLocaleString()} MB`
})
</script>
