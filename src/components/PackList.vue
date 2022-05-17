<template>
<div>
  <PackViewHeader />
  <div class="columns is-multiline" v-if="props.packs.length > 0">
    <div class="column" v-for="pack of props.packs" :key="pack.name">
      <Pack :pack="pack" />
    </div>
  </div>
  <p class="subtitle is-italic has-text-centered my-6 py-6" v-else>
    No packs were found.
  </p>
</div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue'
import { Modpack } from '@/types/Pack'
import PackViewHeader from '@/components/PackViewHeader.vue'
import Pack from '@/components/Pack.vue'

const props = defineProps<{
  packs: Modpack[]
}>()

</script>
