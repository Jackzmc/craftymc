<template>
<div>
  <div class="columns">
    <div class="column is-10">
      <h4 class="title is-4">{{props.pack.name}}</h4>
      <Field :icon-left="['fa', 'search']">
        <!-- TODO: Add 'x' to right on search -->
        <input type="text" class="input" placeholder="Search for mods" v-model="searchQuery" @input="doSearch" />
      </Field>
    </div>
    <div class="column">
      <a class="button is-white is-circular is-pulled-right" @click="emit('close')">
        <fa-icon :icon="['fas', 'times-circle']" size="2x" />
      </a>
    </div>
  </div>
  <Tabs inner-wrapper-class="tabs" :options="{useUrlFragment:false}">
    <Tab name="Mods">
      <FilterControls />
      <p class="has-text-centered mx-5 my-5">
        <span class="has-text-danger" v-if="error">{{error}}</span>
        <span class="subtitle is-4" v-else-if="loading">Loading...</span>
        <span class="subtitle is-4" v-else-if="mods.length == 0">No mods were found.</span>
      </p>
      <EntryCard v-for="entry in mods" :entry="entry" :key="entry.project.id">
      <template v-slot:rightColumn>
          <p v-if="entry.installState == InstallState.Installed">Installed</p>
          <a v-else
            :disabled="entry.installState != InstallState.NotInstalled ? true : undefined"
            :class="['button', 'is-info', {'is-loading': entry.installState == InstallState.Installing}]"
          >Install</a>
      </template>
    </EntryCard>
    </Tab>
    <Tab name="Resource Packs">

    </Tab>

    <Tab name="Maps">

    </Tab>
  </Tabs>
</div>
</template>

<script setup lang="ts">
import { Tab, Tabs } from 'vue3-tabs-component'
import { ModrinthProject } from '@/types/External';
import FilterControls from '@/components/FilterControls.vue'
import EntryCard from '@/components/EntryCard.vue'
import Field from '@/components/form/Field.vue'
import { Modpack, InstallState } from '@/types/Pack';
import { ref } from 'vue'
import { createDebounce } from '@/js/utils'

const emit = defineEmits(["close"])
const props = defineProps<{
  pack: Modpack
}>()

let searchQuery = ref<string>()
let loading = ref(false)
let error = ref<string>()
let mods = ref<ModrinthProject[]>([])

async function searchModrinth() {
  loading.value = true
  error.value = undefined
  try {
    const queryText = searchQuery.value && searchQuery.value != '' ? `&query=${searchQuery.value}` : ''
    const response = await fetch(`https://api.modrinth.com/v2/search?limit=20&index=relevance&facets=[["project_type:mod"]]${queryText}`)
    const json = await response.json()
    if(response.ok) {
      mods.value = (json.hits as ModrinthProject[])
        .map(pack => {
          return {
            project: pack,
            installState: InstallState.NotInstalled
          }
        })
    } else {
      error.value = json.message || json.error
    }
  } catch(err) {
    error.value = err.message
  } finally {
    loading.value = false
  }
}

const doSearch = createDebounce(searchModrinth, 500)

searchModrinth()
</script>
