<template>
<div class="level">
  <div class="level-left">
    <div class="level-item">
      <HorizontalField label="Sort by">
        <div class="select">
          <select v-model="sort">
            <option v-for="(display, key) in props.sorts" :key="key" :value="key">{{display}}</option>
          </select>
        </div>
      </HorizontalField>
    </div>
    <div class="level-item">
      <slot name="filter">
        <HorizontalField label="Filter by">
          <div class="select">
            <select v-model="filter">
              <option v-for="(display, key) in props.filters" :key="key" :value="key">{{display}}</option>
            </select>
          </div>
        </HorizontalField>
      </slot>
    </div>
  </div>
  <div class="level-right" v-if="props.showSize">
    <div class="level-item">
      <input class="slider is-fullwidth has-output" step="1" min="2" max="4" v-model.number="size" type="range" :data-tooltip="sizeName" @input="emit('update:cardsize', size)">
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import HorizontalField from '@/components/form/HorizontalField.vue'

const emit = defineEmits(['update:cardsize', 'update:sort', 'update:filter'])
const props = defineProps<{
  cardsize?: number,
  showSize?: boolean,
  sorts: Record<string, string>,
  filters?: Record<string, string>,
  defaultSort?: string,
  defaultFilter?: string
}>()

//"relevance" "downloads" "follows" "newest" "updated"

let sort = ref(props.defaultSort)
let filter = ref(props.defaultFilter)
let size = ref(props.cardsize || 3)

watch(sort, (value) => emit('update:sort', value))
watch(filter, (value) => emit('update:filter', value))

const sizeName = computed(() => {
  switch(size.value) {
    case 2: return "S"
    case 3: return "M"
    case 4: return "L"
  }
  return "?"
})

</script>
