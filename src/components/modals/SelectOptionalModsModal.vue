<template>
  <BaseModal active title="Welcome!" show-header>
    <ul>
      <li v-for="(i, mod) in props.mods" :key="i">
         <input class="is-checkradio" :id="i" type="checkbox" v-model="selected">
        <label :for="i">{{mod}}</label>
      </li>
    </ul>
    <template v-slot:footer>
      <div class="buttons">
        <a class="button is-success" @click="confirm">
          Install
        </a>
      </div>
    </template>
  </BaseModal>
</template>

<script setup lang="ts">
import BaseModal from './BaseModal.vue'
import { ref } from 'vue'
const emit = defineEmits(['optional-selected'])

const props = defineProps<{
  mods: any[]
}>()

let selected = ref([])

async function confirm() {
  emit('result', selected.value)
}

</script>
