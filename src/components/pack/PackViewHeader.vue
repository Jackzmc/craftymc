<template>
<div class="level">
  <div class="level-left">
    <div class="level-item">
      <HorizontalField label="Sort by">
        <div class="select">
          <select v-model="sort">
            <option v-for="(display, key) in SORTS" :key="key" :value="key">{{display}}</option>
          </select>
        </div>
      </HorizontalField>
    </div>
    <div class="level-item">
      <HorizontalField label="Filter by">
        <div class="select">
          <select v-model="filter">
            <option v-for="(display, key) in FILTERS" :key="key" :value="key">{{display}}</option>
          </select>
        </div>
      </HorizontalField>
    </div>
  </div>
  <div class="level-right">
    <div class="level-item">
      <input class="slider is-fullwidth has-output" step="1" min="2" max="4" v-model.number="size" type="range" :data-tooltip="sizeName" @input="emit('update:cardsize', size)">
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import HorizontalField from '@/components/form/HorizontalField.vue'

const emit = defineEmits(['update:cardsize'])
const props = defineProps<{cardsize: number}>()

const FILTERS = {
  all: "All Modpacks",
  own: "My Modpacks",
  featured: "Featured Modpacks",
  thirdparty: "External Modpacks"
}

const SORTS = {
  recentlyPlayed: "Recently Played",
  mostPlayed: "Most Played",
  name: "Name",
  mcVersion: "Game Version",
  created: "Creation Date"
}

const sort = ref('recentlyPlayed')
const filter = ref('all')
let size = ref(props.cardsize)
const sizeName = computed(() => {
  switch(size.value) {
    case 2: return "S"
    case 3: return "M"
    case 4: return "L"
  }
  return "?"
})

</script>
