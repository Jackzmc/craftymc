<template>
<BaseModal active :title="props.pack.name" @close="close">
  <div class="columns is-mobile is-centered is-vcentered">
    <div class="column is-3">
      <figure class="image is-128x128 is-pulled-left">
        <img :src="props.pack.imageUrl" />
      </figure>
    </div>
    <div class="column">
      <Field label="Name">
        <input class="input" type="text" v-model="pack.name" />
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
import { ref, computed, watch } from 'vue'
const emit = defineEmits(['close'])
const props = defineProps<{
  pack: Modpack
}>()

let pack = ref(props.pack)
watch(pack.value.settings.useCustomMemory, (value) => {
  if(value && pack.value.settings.useCustomMemory) {
    pack.value.settings.javaMemory = 1000 //TODO: Pull form main settings
  }
})

const javaMemory = computed(() => {
  return `${pack.value.settings.javaMemory?.toLocaleString()} MB`
})

function save() {
  emit('save', pack.value)
  emit('close')
}

function close() {
  emit('close')
}
</script>
