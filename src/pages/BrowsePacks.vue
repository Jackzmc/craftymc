<template>
<div>
  <p class="has-text-centered mx-5 my-5">
    <span class="has-text-danger" v-if="error">{{error}}</span>
  </p>
  <PackView :loading="loading" :modpacks="modpacks" @search="searchModrinth" />
</div>
</template>

<script setup lang="ts">
import { ModrinthModpack } from '@/types/External';
import PackView from '@/components/pack/PackView.vue'
import { InstallState } from '@/types/Pack';
import { ref } from 'vue'

let loading = ref(false)
let error = ref<string>()
let modpacks = ref<ModrinthModpack[]>([])

async function searchModrinth(query?: string) {
  loading.value = true
  error.value = undefined
  try {
    const queryText = query && query != '' ? `&query=${query}` : ''
    const response = await fetch(`https://api.modrinth.com/v2/search?limit=20&index=relevance&facets=[["project_type:modpack"]]${queryText}`)
    const json = await response.json()
    if(response.ok) {
      modpacks.value = (json.hits as ModrinthModpack[])
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

searchModrinth()
</script>
