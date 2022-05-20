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
          <select v-model="pack.versions.minecraft">
            <option :value="undefined" v-if="!pack.versions.minecraft">Select a version</option>
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
          <select v-model="pack.settings.modloaderType" @input="getModloaderVersions">
            <option :value="undefined" v-if="!pack.settings.modloaderType">Select a modloader</option>
            <option value="forge">Forge</option>
            <option value="forge">Fabric</option>
          </select>
        </div>
      </Field>
    </div>
    <div class="column">
      <Field label="Modloader Version">
        <div class="select">
          <select disabled v-model="pack.versions.modloader">
            <option :value="undefined" v-if="!pack.versions.modloader">Select a modloader</option>
            <option value="manual">Manually Provided</option>
            <option v-for="version in modloaderVersions" :key="version">
              {{version}}
            </option>
          </select>
        </div>
        <p class="help">Auto download support is not available.</p>
      </Field>
    </div>
  </div>
  <template v-slot:footer>
    <div class="buttons">
      <div :class="['button','is-success',{'is-loading': saving}]" :disabled="saveDisabled" @click="save">Create</div>
      <div class="button" :disabled="saving" @click="close">Cancel</div>
    </div>
  </template>
</BaseModal>
</template>
<script setup lang="ts">
import BaseModal from './BaseModal.vue'
import Field from '@/components/form/Field.vue'
import { Modpack } from '@/types/Pack'
import { ref, computed, onMounted } from 'vue'
import DefaultPackImage from '@/assets/default_pack.png'
import { invoke } from '@tauri-apps/api/tauri'

const emit = defineEmits(['close', 'save'])
let pack = ref<Partial<Modpack>>({
  name: undefined,
  settings: {
    useCustomMemory: false,
    javaMemory: 1000,
    mcVersion: undefined,
    modloaderType: undefined,
  },
  versions: {
    minecraft: undefined,
    modloader: "manual",
    pack: undefined
  },
  lastPlayed: undefined,
  created: undefined
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
let saving = ref(false)

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

}

const saveDisabled = computed(() => {
  if(!saving.value && pack.value.name !== undefined
    && pack.value.versions.minecraft !== undefined
    && pack.value.settings.modloaderType !== undefined)
  {
    return undefined
  }
  return true // Why the fuck does vue3 mean :disabled="false" -> disabled
    // TODO: In future, if/when modloaderVersion supported, check.
})

async function save() {
  saving.value = true
  if(!saveDisabled) return alert("Cannot save: Please fill in all fields.")
  try {
    const savedPack = await invoke('create_modpack', { modpack: pack.value as Modpack })
    emit('save', savedPack)
    saving.value = false
    emit('close')
  } catch(err) {
    console.error('Error creating modpack:', err)
    alert("An error occurred while attempting to save: " + err)
  }
}

function close() {
  emit('close')
}
onMounted(async() => {
  await getMCVersions()
})
</script>
