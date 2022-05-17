<template>
<div>
  <FilterControls v-model:cardsize="cardSize" show-size />
  <div class="columns is-multiline" v-if="props.packs.length > 0">
    <div :class="columnClass" v-for="pack of props.packs" :key="pack.name">
      <Pack :pack="pack" @select="pack => emit('select', pack)" />
    </div>
  </div>
  <p class="subtitle is-italic has-text-centered my-6 py-6" v-else>
    No packs were found.
  </p>
</div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Modpack } from '@/types/Pack'
import FilterControls from '@/components/FilterControls.vue'
import Pack from '@/components/pack/Pack.vue'

const emit = defineEmits(["select"])

const props = defineProps<{
  packs: Modpack[]
}>()

const cardSize = ref(3)

const columnClass = computed(() => {
  return ['column', 'is-multiline', `is-${cardSize.value}`]
})

</script>
