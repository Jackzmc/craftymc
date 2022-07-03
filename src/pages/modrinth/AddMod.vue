<template>
<div>
  <div class="content-nav">
    <div class="columns">
      <div class="column is-10">
        <h4 class="title is-4">{{props.pack.name}}</h4>
        <Field :icon-left="['fa', 'search']">
          <!-- TODO: Add 'x' to right on search -->
          <input type="text" class="input" placeholder="Search for mods" v-model="settings.query" @input="doSearch(false)" />
        </Field>
      </div>
      <div class="column">
        <a class="button is-white is-circular is-pulled-right" @click="emit('close')">
          <fa-icon :icon="['fas', 'times-circle']" size="2x" />
        </a>
      </div>
    </div>
    <div class="tabs">
      <ul>
        <li :class="{'is-active': tabIndex == TabIndex.Mods}" @click="tabIndex = TabIndex.Mods"><a>Mods</a></li>
        <li><a>Resource Packs</a></li>
        <li><a>Maps</a></li>
      </ul>
    </div>
    <FilterControls :sorts="SORTS" :defaultSort="settings.sort"
      @update:sort="(val) => settings.sort = val"
    >
      <template v-slot:filter>
        <p></p>
      </template>
    </FilterControls>
  </div>
  <div style="overflow-y: scroll; height: 70vh; padding-left: 0.1em" ref="contentbody">
    <template v-if="tabIndex == TabIndex.Mods">
      <div class="has-text-centered mx-5 my-5">
        <p class="has-text-danger" v-if="error">
          {{error}}
          <br><br>
          <a class="button is-info" @click="searchModrinth(false)">Refresh</a>
        </p>
        <p class="subtitle is-4" v-else-if="loading">Loading...</p>
        <p class="subtitle is-4" v-else-if="mods.length == 0">No mods were found.</p>
      </div>
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
    </template>
    <div ref="scrollComponent" />
  </div>
</div>
</template>

<script setup lang="ts">
// import { Tab, Tabs } from 'vue3-tabs-component'
import { ModrinthProject } from '@/types/External';
import FilterControls from '@/components/FilterControls.vue'
import EntryCard from '@/components/EntryCard.vue'
import Field from '@/components/form/Field.vue'
import { Modpack } from '@/types/Pack';
import { ref, onBeforeMount, onMounted, computed, onBeforeUnmount, watch } from 'vue'
import { createDebounce } from '@/js/utils'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'

enum TabIndex {
  Mods = 0,
  ResourcePacks = 1,
  Maps = 2,
}

const MAX_FETCH_PER_PAGE = 20

const emit = defineEmits(["close"])
const props = defineProps<{
  pack: Modpack,
  selected?: any
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
let contentbody = ref()
let appVersion = ref()

let tabIndex = ref(TabIndex.Mods)

const settings = ref({
  sort: "relevance",
  categories: [],
  page: 0,
  query: ''
})

watch([() => settings.value.sort, () => settings.value.filter, () => props.selected], () => {
  searchModrinth(false)
})

let categories = ref([])
let loading = ref(false)
let error = ref<string>()

let mods = ref<ModrinthProject[]>([])

async function searchModrinth(nextPage: boolean = false) {
  loading.value = true
  error.value = undefined
  if(nextPage) settings.value.page++
  else {
    settings.value.page = 0
    mods.value = []
  }
  try {
    const facets = [[`categories:${props.pack.settings.modloaderType}`],[`project_type:mod`],[`versions:${props.pack.versions.minecraft}`]]
    // const facets = ["project_type:mod"]
    if(props.selected && props.selected > 0) {
      const categoryFacet = []
      for(const category of props.selected) {
        categoryFacet.push(`categories:${category}`)
      }
      facets.push(categoryFacet)
    }
    const facetsString = `[["${facets.join('"],["')}"]]`
    const queryText = settings.value.query != '' ? `&query=${settings.value.query}` : ''
    const offset = MAX_FETCH_PER_PAGE *settings.value.page

    const response = await fetch(`https://api.modrinth.com/v2/search?limit=${MAX_FETCH_PER_PAGE}&offset=${offset}&index=${settings.value.sort}&facets=${facetsString}${queryText}`, {
    headers: {
      'User-Agent': `Jackzmc/CraftyMc v${appVersion.value}`
    }
  })
    // TODO: Check versions to see if there is a valid version FOR the modloader
    // OR wait until modrinth fixes it on their side
    const json = await response.json()
    if(response.ok) {
      mods.value = mods.value.concat((json.hits as ModrinthProject[])
        .map(pack => {
          return {
            project: pack,
            installing: false
          }
        })
      )
    } else {
      error.value = json.description || json.message || json.error
    }
  } catch(err) {
    error.value = err.message
  } finally {
    loading.value = false
  }
}

async function getCategories() {
  const response = await fetch(`https://api.modrinth.com/v2/tag/category`, {
    headers: {
      'User-Agent': `Jackzmc/CraftyMc v${appVersion.value}`
    }
  })
  const json = await response.json()
  if(response.ok) {
    return json
  } else {
    throw new Error(json.description || json.message || json.error)
  }
}

async function installMod(entry: Entry) {
  entry.installing = true
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
  const response = await fetch(`https://api.modrinth.com/v2/project/${entry.project.slug}/version?loaders=["${props.pack.settings.modloaderType}"]&game_versions=["${props.pack.versions.minecraft}"]`, {
    headers: {
      'User-Agent': `Jackzmc/CraftyMc v${appVersion.value}`
    }
  })
  const json = await response.json()
  if(response.ok) {
    return json as ModrinthProjectVersion[]
  } else {
    throw new Error(json.description || json.message || json.error)
  }
}

const doSearch = createDebounce(searchModrinth, 500)
const scrollSearch = createDebounce(searchModrinth, 1500)

onBeforeMount(async() => {
  appVersion.value = await getVersion()
  categories.value = await getCategories()
  searchModrinth()
  listen("download-mod", async(event) => {
    if(event.payload.error) {
      alert(event.payload.error)
    } else {
      // Don't do anything, will send update-modload event
    }
  })
})

onMounted(() => {
  contentbody.value.addEventListener("scroll", handleScroll)
})

onBeforeUnmount(() => {
  contentbody.value.removeEventListener("scroll", handleScroll)
})

const earlyScrollAmount = 150
function handleScroll() {
  if (scrollComponent.value.getBoundingClientRect().bottom <= contentbody.value.clientHeight + contentbody.value.getBoundingClientRect().top + earlyScrollAmount) {
    scrollSearch(true)
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

<style>
.content-nav {
  height: 14em;
  overflow-x: hidden;
  overflow-y: clip;
}
html, body {
  overflow: hidden;
}
/* Scrollbar styles */
::-webkit-scrollbar {
width: 12px;
height: 12px;
}

::-webkit-scrollbar-track {
border: 1px solid #3e8ed0;
border-radius: 2px;
}

::-webkit-scrollbar-thumb {
background: #3e8ed0;
border-radius: 50px;
}

::-webkit-scrollbar-thumb:hover {
background: #88ba1c;
}
</style>
