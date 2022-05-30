<template>
<div>
  <component :is="component" :pack="componentPack" v-if="component" @close="closeComponent" />
  <FilterControls :sorts="SORTS" defaultSort="recentlyPlayed" :filters="FILTERS" defaultFilter="all"
    v-model:cardsize="cardSize" show-size @update:sort="(val) => packSort = val"  @update:filter="(val) => packFilter = val"
  />
  <div class="columns is-multiline" v-if="props.packs.length > 0">
    <div :class="columnClass" v-for="pack of sortedPacks" :key="pack.id">
      <Pack :pack="pack" @select="pack => emit('select', pack)" @contextmenu.prevent="event => contextMenu.open(event, pack)" />
    </div>
    <div :class="columnClass" v-for="pack of invalidModpacks" :key="pack.name">
      <InvalidPack :name="pack.name" :reason="pack.reason" />
    </div>
  </div>
  <p class="subtitle is-italic has-text-centered my-6 py-6" v-else>
    No packs were found.
  </p>
  <ContextMenu class="box" ref="contextMenu">
    <template v-slot="slotProps"> <!-- slotsProps are props passed from child to slot in parent -->
      <!-- Now, here slotProps.ctx does contain the context (cat) BUT -->
      <template v-if="slotProps.ctx"> <!-- we need to add this since context is null on initial render -->
        <!-- Now we can peacefully access slotProps.ctx without getting any errors -->
        <h5 class="title is-5">{{slotProps.ctx.name}}</h5>
          <ul class="menu-list">
            <li><a class="has-text-info" @click="launch(slotProps.ctx)">
              <Icon :icon="['fa', 'play']" text="Play" />
            </a></li>
            <li><a @click="emit('edit', slotProps.ctx); contextMenu.close()">
              <Icon :icon="['fa', 'pencil']" text="Edit" />
            </a></li>
            <li><a @click="invoke('open_modpack_folder', { packId: slotProps.ctx.id }); contextMenu.close()">
              <Icon :icon="['fa', 'folder']" text="Open Folder" />
            </a></li>
            <li><a @click="openExportMenu(slotProps.ctx)">
              <Icon :icon="['fa', 'file-export']" text="Export" />
            </a></li>
            <li><a class="has-text-danger" @click="showDeleteConfirm(slotProps.ctx)">
              <Icon :icon="['fa', 'trash']" text="Delete" />
            </a></li>
          </ul>
      </template>
    </template>

  </ContextMenu>
</div>
</template>

<script setup lang="ts">
import { ref, computed, defineAsyncComponent, markRaw } from 'vue'
import { Modpack } from '@/types/Pack'
import FilterControls from '@/components/FilterControls.vue'
import Pack from '@/components/pack/Pack.vue'
import InvalidPack from '@/components/pack/InvalidPack.vue'
import ContextMenu from '@/components/ContextMenu.vue'
import { invoke } from '@tauri-apps/api/tauri'

let component = ref()
let componentPack = ref()
let packFilter = ref("all")
let packSort = ref("recentlyPlayed")

const emit = defineEmits(["select", "edit"])

const props = defineProps<{
  packs: Modpack[],
  invalidModpacks: any[]
}>()

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

const sortedPacks = computed(() => {
  return [...props.packs]
  .filter(pack => {
    switch(packFilter.value) {
      case "own":
        return !pack.author
      case "featured":
        return false
      case "thirdparty":
        return pack.versions.pack
      case "all":
      default:
         return true
    }
  })
  .sort((a,b) => {
    switch(packSort.value) {
      case "recentlyPlayed":
        return new Date(b.lastPlayed) - new Date(a.lastPlayed)
      case "mostPlayed":
        return b.timesPlayed - a.timesPlayed
      case "name":
        return a.name.localeCompare(b.name)
      case "mcVersion":
        return a.versions.minecraft.localeCompare(b.versions.minecraft)
      case "created":
        return new Date(b.created) - new Date(a.created)
    }
  })
})

const cardSize = ref(3)
let contextMenu = ref()

const columnClass = computed(() => {
  return ['column', 'is-multiline', `is-${cardSize.value}`]
})

async function showDeleteConfirm(pack: Modpack) {
  component.value = markRaw(defineAsyncComponent(() => import('@/components/modals/DeletePackConfirmModal.vue')))
  componentPack.value = pack
  contextMenu.value.close()
}

async function launch(pack: Modpack) {
  await invoke('launch_modpack', { id: pack.id })
  contextMenu.value.close()
}

function openExportMenu(pack: Modpack) {
  component.value = markRaw(defineAsyncComponent(() => import('@/components/modals/ExportModal.vue')))
  componentPack.value = pack
  contextMenu.value.close()
  // invoke('export_modpack', { packId: pack.id, fileName: `${pack.folder}.zip` })
}

function closeComponent() {
  component.value = undefined
}

</script>
