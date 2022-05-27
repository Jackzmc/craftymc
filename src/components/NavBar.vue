<template>
<div>
<CreatePackModal v-if="showCreatePack" active @close="showCreatePack = false" @save="onModpackCreated" />
<nav class="navbar is-black is-fixed-top" role="navigation" aria-label="main navigation">
  <div id="navbarBasicExample" class="navbar-menu" data-tauri-drag-region>
    <div class="navbar-start">
      <a @click="emit('sidebar')" class="navbar-item">
        <Icon :icon="sidebarIcon" text="CraftyMc Mod Manager" class="has-tooltip-bottom" data-tooltip="Toggles the sidebar" />
      </a>
    </div>

    <div class="navbar-end">
      <div class="navbar-item">
        <div class="buttons">
          <a class="button">
            <Icon :icon="['fas', 'plus']" text="Import" @click="importModpack" />
          </a>
          <a class="button is-info">
            <Icon :icon="['fas', 'plus']" text="New Modpack" @click="showCreatePack = true" />
          </a>
          <a class="button is-black">
            <Icon :icon="['fas', 'window-minimize']" @click="appWindow.minimize()" />
          </a>
          <a class="button is-black">
            <Icon :icon="['far', 'square']" @click="appWindow.maximize()"  />
          </a>
          <a class="button is-black">
            <Icon :icon="['fa', 'close']" @click="appWindow.close()" />
          </a>
        </div>
      </div>
    </div>
  </div>
</nav>
</div>
</template>

<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'
import { computed, ref } from 'vue'
import CreatePackModal from '@/components/modals/CreatePackModal.vue'
import { invoke } from '@tauri-apps/api/tauri'
// eslint-disable-next-line
const props = defineProps<{
  hasSidebar: boolean
}>()
const emit = defineEmits(["sidebar", "installModloader"])

let showCreatePack = ref(false)

async function onModpackCreated(pack) {
  // await emit('update-modpacks', pack)
  await emit('installModloader', pack)
}

async function importModpack() {
  await invoke("import_modpack", {})
}

const sidebarIcon = computed(() => {
  return [
    'fas',
    // 'bars'
    props.hasSidebar ? 'circle-check' : 'circle'
  ]
})
</script>
