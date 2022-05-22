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
      <FilterControls :sorts="SORTS" defaultSort="relevance" @update:sort="(val) => sort = val"  @update:filter="(val) => filter = val" />
      <p class="has-text-centered mx-5 my-5">
        <span class="has-text-danger" v-if="error">{{error}}</span>
        <span class="subtitle is-4" v-else-if="loading">Loading...</span>
        <span class="subtitle is-4" v-else-if="mods.length == 0">No mods were found.</span>
      </p>
      <EntryCard v-for="entry in mods" :entry="entry" :key="entry.project.project_id">
        <template v-slot:rightColumn>
            <p v-if="installedMods[entry.project.project_id]">Installed</p> <!-- remove pack.mods once rust has it -->
            <a v-else
              :disabled="entry.installing || undefined"
              :class="['button', 'is-info', {'is-loading': entry.installing }]"
              @click="installMod(entry)"
            >Install</a>
        </template>
      </EntryCard>
      <div ref="scrollComponent" />
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
import { Modpack } from '@/types/Pack';
import { ref, onBeforeMount, computed, onUnmounted } from 'vue'
import { createDebounce } from '@/js/utils'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'

const MAX_FETCH_PER_PAGE = 20

const emit = defineEmits(["close"])
const props = defineProps<{
  pack: Modpack
}>()

const installedMods = computed(() => {
  let rec: Record<String, SavedModEntry> = {}
  for(const mod of props.pack.mods) {
    rec[mod.project_id] = mod
  }
  return rec
}, { deep: true })

const SORTS = computed(() => {
  if(props.pack.settings.modSource === "modrinth") {
    return {
      relevance: "Relevance",
      downloads: "Downloads",
      follows: "Follows",
      newest: "Newest",
      updated: "Updated"
    }
  } else {
    console.warn("Unsupported or unknown modsource", props.pack.settings.modsource)
    return []
  }
})

let scrollComponent = ref()
let sort = ref('relevance')
let filter = ref('all')
let debug = ref<string>()
let searchQuery = ref<string>()
let loading = ref(false)
let error = ref<string>()
let mods = ref<ModrinthProject[]>([])
let page = ref(0)

async function searchModrinth(nextPage: boolean = false) {
  loading.value = true
  error.value = undefined
  if(nextPage) page.value++
  try {
    const facets = ["project_type:mod"]
    facets.push(`categories:${props.pack.settings.modloaderType}`)
    facets.push(`versions:${props.pack.versions.minecraft}`)
    const facetsString = `[["${facets.join('"],["')}"]]`

    const queryText = searchQuery.value && searchQuery.value != '' ? `&query=${searchQuery.value}` : ''
    const offset = MAX_FETCH_PER_PAGE * page.value
    const response = await fetch(`https://api.modrinth.com/v2/search?limit=${MAX_FETCH_PER_PAGE}&offset=${offset}&index=${sort.value}&facets=${facetsString}${queryText}`)
    // TODO: Check versions to see if there is a valid version FOR the modloader
    // OR wait until modrinth fixes it on their side
    const json = await response.json()
    if(response.ok) {
      debug.value = {...json, _url: `https://api.modrinth.com/v2/search?limit=20&index=relevance&facets=${facetsString}${queryText}`}
      mods.value = mods.value.concat((json.hits as ModrinthProject[])
        .map(pack => {
          return {
            project: pack,
            installing: false
          }
        })
      )
    } else {
      error.value = json.message || json.error
    }
  } catch(err) {
    error.value = err.message
  } finally {
    loading.value = false
  }
}

async function installMod(entry: Entry) {
  entry.installing = true
  console.log(entry.project)
  const versions = (await getModVersions(entry))
    .filter((version) => {
      // TODO: Check against settings for version_type
      return version.version_type && true
    })
    .sort((a,b) => new Date(b.datePublished) - new Date(a.datePublished))
  console.debug('versions', versions)
  if(versions.length == 0) {
    entry.installing = false
    console.warn(`Could not find versions for mod.`, entry.project)
    return alert("Could not find any valid versions. Probably a bug. Mod id:", entry.project.id)
  }
  await invoke('install_mod', {
    packId: props.pack.id,
    modId: entry.project.id,
    authorName: entry.project.author,
    versionData: versions[0]
  })
}

async function getModVersions(entry: Entry): Promise<ModrinthProjectVersion[]> {
  const response = await fetch(`https://api.modrinth.com/v2/project/${entry.project.slug}/version?loaders=["${props.pack.settings.modloaderType}"]&game_versions=["${props.pack.versions.minecraft}"]`)
  const json = await response.json()
  if(response.ok) {
    return json as ModrinthProjectVersion[]
  } else {
    throw new Error(json.message || json.error)
  }
}

const doSearch = createDebounce(searchModrinth, 500)

onBeforeMount(() => {
  searchModrinth()
  listen("download-mod", (event) => {
    if(event.payload.error) {
      alert(event.payload.error)
    } else {
      // Don't do anything, will send update-modload event
    }
    console.log(event)
  })
  window.addEventListener("scroll", handleScroll)
})

onUnmounted(() => {
  window.removeEventListener("scroll", handleScroll)
})

function handleScroll() {
  if (scrollComponent.value.getBoundingClientRect().bottom < window.innerHeight) {
    doSearch(true)
  }
}

/*interface DownloadSuccess {
  mod_id: string,
  pack_id: string,
  file_name: string,
  error: string,
}
interface DownloadFailure {
  mod_id: string,
  pack_id: string,
}*/

</script>
