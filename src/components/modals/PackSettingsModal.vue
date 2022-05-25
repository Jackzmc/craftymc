<template>
<BaseModal active title="Edit modpack" @close="close" show-header>
  <div class="columns is-mobile is-centered is-vcentered">
    <div class="column is-3 has-tooltip-bottom" data-tooltip="Click to change">
      <figure class="image is-128x128">
        <img :src="props.pack.imageUrl" @click="choosePackImage" />
      </figure>
    </div>
    <div class="column">
      <Field label="Name">
        <input class="input" type="text" v-model="pack.name" @input="updateName" />
      </Field>
    </div>
  </div>
  <Field label="Java Memory Settings">
    <input class="is-checkradio" id="enableJavaMemory" type="checkbox" name="exampleCheckbox" v-model="pack.settings.useCustomMemory">
    <label for="enableJavaMemory">Use custom memory settings</label>
    <input v-if="pack.settings.useCustomMemory" id="pack-javamemslider" class="slider is-fullwidth has-output" step="250" min="1000" max="8000" v-model.number="pack.settings.javaMemory" type="range" :data-tooltip="javaMemory">
  </Field>
  <template v-slot:footer>
    <div class="buttons">
      <div class="button is-success" @click="save">Save Changes</div>
      <div class="button" @click="close">Cancel</div>
    </div>
  </template>
</BaseModal>
</template>
<script setup lang="ts">
import BaseModal from './BaseModal.vue'
import Field from '@/components/form/Field.vue'
import { Modpack } from '@/types/Pack'
import { reactive, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const emit = defineEmits(['close'])
const props = defineProps<{
  pack: Modpack
}>()

let pack = reactive<Modpack>(JSON.parse(JSON.stringify(props.pack)))

watch(pack.settings, () => {
  for(const key in pack.settings) {
    if(props.settings[key] !== settings.value[key]) {
      invoke('set_setting', {
        packId: pack.id,
        key,
        value: settings.value[key].toString()
      })
    }
  }
}, { deep: true })

async function updateName() {
  await invoke('set_modpack_setting', {
    packId: pack.id,
    key: 'name',
    value: pack.name
  })
}

const javaMemory = computed(() => {
  return `${pack.settings.javaMemory?.toLocaleString()} MB`
})

async function choosePackImage() {
  await invoke("choose_modpack_image", { packId: props.pack.id })
}


async function save() {
  await invoke('save_modpack', { packId: pack.id })
  emit('close')
}

function close() {
  emit('close')
}
</script>
