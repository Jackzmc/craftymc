<template>
<BaseModal active show-header :title="'Deleting ' + props.pack.name">
  <p class="subtitle is-4">Are you sure you want to delete this modpack?</p>
  <p>This will delete any saved backups</p>
  <template v-slot:footer>
    <div class="buttons">
      <a :class="['button', 'is-danger', {'is-loading': deleting}]" @click="confirm(true)">Delete</a>
      <a :class="['button', {'is-loading': deleting}]" @click="confirm(false)">Cancel</a>
    </div>
  </template>
</BaseModal>
</template>

<script setup lang="ts">
import { Modpack } from '@/types/Pack'
import BaseModal from './BaseModal.vue'
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'
const emit = defineEmits(["close"])
const props = defineProps<{
  pack: Modpack
}>()

let deleting = ref(false)

async function confirm(result: boolean) {
  if(result) {
    deleting.value = true
    try {
      await invoke("delete_modpack", {
        packId: props.pack.id
      })
    } catch(err) {
      alert("Failed to delete modpack: " + err)
    }
    deleting.value = false
  }
  emit("close")
}
</script>
