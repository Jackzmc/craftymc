<template>
<div class="level">
  <div class="level-left">
    <div class="level-item">
      <HorizontalField title="Sort by">
        <div class="select">
          <select v-model="sort">
            <option v-for="(display, key) in SORTS" :key="key" :value="key">{{display}}</option>
          </select>
        </div>
      </HorizontalField>
    </div>
    <div class="level-item">
      <HorizontalField title="Filter by">
        <div class="select">
          <select v-model="filter">
            <option v-for="(display, key) in FILTERS" :key="key" :value="key">{{display}}</option>
          </select>
        </div>
      </HorizontalField>
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import HorizontalField from '@/components/HorizontalField.vue'

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
</script>
