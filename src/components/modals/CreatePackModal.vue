<template>
<BaseModal active title="New Modpack" @close="close" show-header>
  <div class="columns is-mobile is-centered is-vcentered">
    <div class="column is-3">
      <img :src="pack.imageUrl || DefaultPackImage" />
    </div>
    <div class="column">
      <Field label="Name">
        <input class="input" type="text" v-model="pack.name" />
      </Field>
    </div>
  </div>
  <Field label="Java Memory Settings">
    <input class="is-checkradio" id="enableJavaMemoryModpack" type="checkbox" name="exampleCheckbox" v-model="pack.settings.useCustomMemory">
    <label for="enableJavaMemoryModpack">Use custom memory settings</label>
    <input v-if="pack.settings.useCustomMemory" id="pack-javamemslider" class="slider is-fullwidth has-output" step="250" min="1000" max="8000" v-model.number="pack.settings.javaMemory" type="range" :data-tooltip="javaMemory">
  </Field>
  <div class="columns">
    <div class="column">
      <Field label="Minecraft Version">
        <div class="select">
          <select v-model="pack.settings.mcVersion">
            <option v-for="version in shownMcVersions" :key="version.version">
              {{version.version}}
            </option>
          </select>
        </div>
      </Field>
    </div>
    <div class="column">
      <Field label="Modloader">
        <div class="select">
          <select v-model="pack.settings.modloader" @input="getModloaderVersions">
            <option :value="undefined" v-if="!pack.settings.modloader">Select a modloader</option>
            <option value="forge">Forge</option>
            <option value="forge">Fabric</option>
          </select>
        </div>
      </Field>
    </div>
    <div class="column">
      <Field label="Modloader Version">
        <div class="select">
          <select v-model="pack.settings.modloaderVersion">
            <option :value="undefined" v-if="!pack.settings.modloader">Select a modloader</option>
            <option v-for="version in modloaderVersions" :key="version">
              {{version}}
            </option>
          </select>
        </div>
      </Field>
    </div>
  </div>
  <template v-slot:footer>
    <div class="buttons">
      <div class="button is-success" @click="save">Create</div>
      <div class="button" @click="close">Cancel</div>
    </div>
  </template>
</BaseModal>
</template>
<script setup lang="ts">
 /* eslint-disable */
import BaseModal from './BaseModal.vue'
import Field from '@/components/form/Field.vue'
import { Modpack } from '@/types/Pack'
import { ref, computed, onMounted, watch } from 'vue'
import DefaultPackImage from '@/assets/default_pack.png'

const emit = defineEmits(['close'])
let pack = ref<Partial<Modpack>>({
  settings: {
    useCustomMemory: false,
    javaMemory: 1000,
    mcVersion: undefined,
    modloader: undefined,
    modloaderVersion: undefined,
  }
})

const javaMemory = computed(() => {
  return `${pack.value.settings.javaMemory?.toLocaleString()} MB`
})

interface MCVersion {
  version: string,
  version_type: string,
  date: string
  major: boolean
}

let mcVersions = ref<MCVersion[]>([])
let modloaderVersions = ref<any[]>([])

const shownMcVersions = computed(() => {
  return mcVersions.value.filter(version => version.version_type === "release")
})

async function getMCVersions() {
  const response = await fetch("https://api.modrinth.com/v2/tag/game_version")
  const json = await response.json()
  if(response.ok) {
    mcVersions.value = json as MCVersion[]
  }
}

async function getModloaderVersions() {
  const response = await fetch("https://api.modrinth.com/v2/tag/game_version")
  const json = await response.json()
  if(response.ok) {
    modloaderVersions.value = json as MCVersion[]
  }
}

function save() {
  emit('save', pack.value)
  emit('close')
}

function close() {
  emit('close')
}
onMounted(async() => {
  await getMCVersions()
})
</script>
