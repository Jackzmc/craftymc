<template>
<div>
  <component :is="component" :pack="componentPack" v-if="component" />
  <FilterControls :sorts="SORTS" defaultSort="recentlyPlayed" v-model:cardsize="cardSize" show-size />
  <div class="columns is-multiline" v-if="props.packs.length > 0">
    <div :class="columnClass" v-for="pack of props.packs" :key="pack.id">
      <Pack :pack="pack" @select="pack => emit('select', pack)" @contextmenu.prevent="event => contextMenu.open(event, pack)" />
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
            <li><a @click="emit('select', slotProps.ctx); contextMenu.close()">
              <Icon :icon="['fa', 'pencil']" text="Edit" />
            </a></li>
            <li><a @click="invoke('open_modpack_folder', { packId: slotProps.ctx.id })">
              <Icon :icon="['fa', 'folder']" text="Open Folder" />
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
import { ref, computed, defineAsyncComponent } from 'vue'
import { Modpack } from '@/types/Pack'
import FilterControls from '@/components/FilterControls.vue'
import Pack from '@/components/pack/Pack.vue'
import ContextMenu from '@/components/ContextMenu.vue'
import { invoke } from '@tauri-apps/api/tauri'

let component = ref()
let componentPack = ref()

const emit = defineEmits(["select"])

const props = defineProps<{
  packs: Modpack[]
}>()

const SORTS = {
  recentlyPlayed: "Recently Played",
  mostPlayed: "Most Played",
  name: "Name",
  mcVersion: "Game Version",
  created: "Creation Date"
}

const cardSize = ref(3)
let contextMenu = ref()

const columnClass = computed(() => {
  return ['column', 'is-multiline', `is-${cardSize.value}`]
})

async function showDeleteConfirm(pack: Modpack) {
  component.value = defineAsyncComponent(() => import('@/components/modals/DeletePackConfirmModal.vue'))
  componentPack.value = pack
  contextMenu.value.close()
}

async function launch(pack: Modpack) {
  // TODO: Show that launcher is running
  const exitCode = await invoke('launch_modpack', { id: pack.id })
  console.info('launched modpack exited with code', exitCode)
  contextMenu.value.close()
}

</script>
