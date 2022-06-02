<template>
<div>
  <div class="columns">
    <div class="column is-10">
      <h4 class="title is-4">Find a modpack</h4>
      <Field :icon-left="['fa', 'search']">
        <!-- TODO: Add 'x' to right on search -->
        <input type="text" class="input" placeholder="Search for a modpack" v-model="search" @input="sendSearch" />
      </Field>
    </div>
  </div>
  <FilterControls :sorts="SORTS" defaultSort="recentlyPlayed" />
  <p class="has-text-centered my-6 subtitle is-4" v-if="loading">Loading...</p>
  <p class="has-text-centered my-6 subtitle is-4" v-else-if="modpacks.length == 0">No packs were found.</p>
  <EntryCard v-for="entry in props.modpacks" :entry="entry" :key="entry.project.project_id">
    <template v-slot:rightColumn>
        <a
          :disabled="entry.installState != InstallState.NotInstalled ? true : undefined"
          :class="['button', 'is-info', {'is-loading': entry.installState == InstallState.Installing}]"
          @click="installModpack"
        >Install</a>
    </template>
  </EntryCard>
</div>
</template>

<script setup lang="ts">
import { Entry } from '@/types/External';
import FilterControls from '@/components/FilterControls.vue'
import EntryCard from '@/components/EntryCard.vue'
import Field from '@/components/form/Field.vue'
import { InstallState } from '@/types/Pack';
import { createDebounce } from '@/js/utils';
import { ref } from 'vue'

const SORTS = {
  recentlyPlayed: "Recently Played",
  mostPlayed: "Most Played",
  name: "Name",
  mcVersion: "Game Version",
  created: "Creation Date"
}

const emit = defineEmits(['search'])
const props = defineProps<{
  modpacks: Entry[],
  loading?: boolean
}>()


let search = ref<string>()
const sendSearch = createDebounce(() => {
  emit('search', search.value)
}, 500)

function installModpack(pack: Entry) {

}
</script>
