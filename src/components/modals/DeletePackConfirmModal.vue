<template>
<BaseModal active show-header :title="'Deleting ' + props.pack.name">
  <p class="subtitle is-4">Are you sure you want to delete this modpack?</p>
  <p>This will delete any saved backups</p>
  <template v-slot:footer>
    <div class="buttons">
      <a class="button is-danger" @click="confirm(true)">Delete</a>
      <a class="button" @click="confirm(false)">Cancel</a>
    </div>
  </template>
</BaseModal>
</template>

<script setup lang="ts">
import { Modpack } from '@/types/Pack'
import BaseModal from './BaseModal.vue'
import { invoke } from '@tauri-apps/api/tauri'
const emit = defineEmits(["cancel"])
const props = defineProps<{
  pack: Modpack
}>()


async function confirm(result: boolean) {
  if(result) {
    await invoke("delete_modpack", {
      packId: props.pack.id
    })
  }
  emit("cancel")
}
</script>
