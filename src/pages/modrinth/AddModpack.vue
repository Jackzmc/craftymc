<template>
<div>
  <div class="content-nav">
    <div class="columns">
      <div class="column is-10">
        <h4 class="title is-4">Browse Modpacks</h4>
        <Field :icon-left="['fa', 'search']">
          <!-- TODO: Add 'x' to right on search -->
          <input type="text" class="input" placeholder="Search for modpacks" v-model="settings.query" @input="doSearch(false)" />
        </Field>
      </div>
      <div class="column">
        <a class="button is-white is-circular is-pulled-right" @click="emit('close')">
          <fa-icon :icon="['fas', 'times-circle']" size="2x" />
        </a>
      </div>
    </div>
    <div class="level">
      <div class="level-left">
        <div class="level-item">
          <HorizontalField label="Sort by">
            <div class="select">
              <select v-model="settings.sort">
                <option v-for="(display, key) in SORTS" :key="key" :value="key">{{display}}</option>
              </select>
            </div>
          </HorizontalField>
        </div>
        <div class="level-item">
          <HorizontalField label="Minecraft Version">
            <div class="select">
              <select v-model="settings.minecraft">
                <option :value="undefined">Any</option>
                <option v-for="version in mcVersions" :key="version.version" :value="version.version">{{version.version}}</option>
              </select>
            </div>
          </HorizontalField>
        </div>
        <div class="level-item">
          <HorizontalField label="Modloader">
            <div class="select">
              <select v-model="settings.modloader">
                <option :value="undefined">Any</option>
                <option v-for="modloader in MODLOADERS" :key="modloader" :value="modloader.toLowerCase()">{{modloader}}</option>
              </select>
            </div>
          </HorizontalField>
        </div>
      </div>
    </div>
  </div>
  <div style="overflow-y: scroll; height: 78vh; padding-left: 0.1em" ref="contentbody">
    <EntryCard v-for="entry in modpacks" :entry="entry" :key="entry.project.project_id">
      <template v-slot:rightColumn>
          <a
            :disabled="entry.installing || undefined"
            :class="['button', 'is-info', {'is-loading': entry.installing }]"
            @click="installModpack(entry)"
          >Install</a>
      </template>
    </EntryCard>
    <div ref="scrollComponent" />
  </div>
</div>
</template>

<script setup lang="ts">
import { ModrinthModpack } from '@/types/External';
import Field from '@/components/form/Field.vue'
import { ref, onBeforeMount, onMounted, computed, onBeforeUnmount, watch } from 'vue'
import { createDebounce } from '@/js/utils'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import EntryCard from '@/components/EntryCard.vue'
import HorizontalField from '@/components/form/HorizontalField.vue'
import { useRouter } from 'vue-router';

const router = useRouter()

const MAX_FETCH_PER_PAGE = 20

const emit = defineEmits(["close", "show"])
const props = defineProps<{
  selected: any
}>()

const SORTS = computed(() => {
  return {
    relevance: "Relevance",
    downloads: "Downloads",
    follows: "Follows",
    newest: "Newest",
    updated: "Updated"
  }

})

const MODLOADERS = [
  "Forge",
  "Fabric"
]


let scrollComponent = ref()
let contentbody = ref()

const settings = ref({
  sort: "relevance",
  categories: [],
  minecraft: undefined,
  modloader: undefined,
  page: 0,
  query: ''
})

watch([() => settings.value.sort, () => settings.value.minecraft, () => props.selected], () => {
  searchModrinth(false)
})

let categories = ref([])
let loading = ref(false)
let error = ref<string>()
let mcVersions = ref([])

let modpacks = ref<ModrinthProject[]>([])

async function searchModrinth(nextPage: boolean = false) {
  loading.value = true
  error.value = undefined
  if(nextPage) settings.value.page++
  else {
    settings.value.page = 0
    modpacks.value = []
  }
  try {
    const facets = [[`project_type:modpack`]]
    if(settings.value.modloader) {
      facets.push([`categories:${settings.value.modloader}`])
    }
    if(settings.value.minecraft) {
      facets.push([`versions:${settings.value.minecraft}`])
    }
    console.log(props.selecte)
    if(props.selected && props.selected.length > 0) {
      const categoryFacet = []
      for(const category of props.selected) {
        categoryFacet.push(`categories:${category}`)
      }
      facets.push(categoryFacet)
    }
    const facetsString = `[["${facets.join('"],["')}"]]`
    const queryText = settings.value.query != '' ? `&query=${settings.value.query}` : ''
    const offset = MAX_FETCH_PER_PAGE *settings.value.page

    const response = await fetch(`https://api.modrinth.com/v2/search?limit=${MAX_FETCH_PER_PAGE}&offset=${offset}&index=${settings.value.sort}&facets=${facetsString}${queryText}`)
    // TODO: Check versions to see if there is a valid version FOR the modloader
    // OR wait until modrinth fixes it on their side
    const json = await response.json()
    if(response.ok) {
      modpacks.value = modpacks.value.concat((json.hits as ModrinthModpack[])
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

async function getMCVersions() {
  const response = await fetch("https://api.modrinth.com/v2/tag/game_version")
  const json = await response.json()
  if(response.ok) {
    mcVersions.value = json.filter(v => v.version_type === "release") as MCVersion[]
  }
}

async function getCategories() {
  const response = await fetch(`https://api.modrinth.com/v2/tag/category`)
  const json = await response.json()
  if(response.ok) {
    return json
  } else {
    throw new Error(json.description || json.message || json.error)
  }
}

async function installModpack(entry: Entry) {
  entry.installing = true
  const versions = (await getVersions(entry))
    .filter((version) => {
      // TODO: Check against settings for version_type
      return version.version_type && true
    })
    .sort((a,b) => new Date(b.datePublished) - new Date(a.datePublished))
  if(versions.length == 0) {
    entry.installing = false
    console.warn(`Could not find versions for modpack.`, entry.project)
    return alert("Could not find any valid versions. Probably a bug. Modpack id:", entry.project.id)
  }
  console.debug('project', entry.project, 'version', versions[0], {
    projectId: entry.project.project_id,
    authorName: entry.project.author,
    versionData: versions[0]
  })

  invoke('install_modpack', {
    projectId: entry.project.project_id,
    authorName: entry.project.author,
    versionData: versions[0]
  })
  entry.installing = false
  router
  // router.push('/')
}

async function getVersions(entry: Entry): Promise<ModrinthProjectVersion[]> {
  const loadersStr = settings.value.modloader ? `&loaders=["${settings.value.modloader}"]` : ''
  const mcVersionStr = settings.value.minecraft ? `&game_versions=["${settings.value.minecraft}"]` : ''
  const response = await fetch(`https://api.modrinth.com/v2/project/${entry.project.slug}/version?${loadersStr}${mcVersionStr}`)
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
  emit('show', { type: 'category-picker', for: 'modpack' })
  categories.value = await getCategories()
  getMCVersions();
  listen("download-mod", async(event) => {
    if(event.payload.error) {
      alert(event.payload.error)
    } else {
      // Don't do anything, will send update-modload event
    }
  })
})

onMounted(() => {
  searchModrinth()
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
  height: 10em;
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
