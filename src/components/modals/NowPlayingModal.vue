<template>
<BaseModal active hide-footer>
  <div class="has-text-centered mx-5 my-6">
    <h4 class="title is-4">Game is running</h4>
    <p class="subtitle is-6">{{props.pack.name}}</p>
  </div>
  <template v-slot:footer>
  </template>
</BaseModal>
</template>

<script setup lang="ts">
import BaseModal from './BaseModal.vue'
import { Modpack } from '@/types/Pack'
const props = defineProps<{
  pack: Modpack
}>()
</script>
