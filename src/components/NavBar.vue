<template>
<div>
<CreatePackModal v-if="showCreatePack" active @close="showCreatePack = false" @save="onModpackCreated" />
<ModloaderInstaller v-if="pendingModpackInstall" :pack="pendingModpackInstall" active  @save="pendingModpackInstall = undefined" />
<nav class="navbar is-black is-fixed-top" role="navigation" aria-label="main navigation">
  <div id="navbarBasicExample" class="navbar-menu" data-tauri-drag-region>
    <div class="navbar-start">
      <a @click="emit('sidebar')" class="navbar-item">
        <span class="icon-text">
          <span class="icon">
            <fa-icon :icon="sidebarIcon" />
          </span>
          <span>Minecraft Mod Manager</span>
        </span>
      </a>
    </div>

    <div class="navbar-end">
      <div class="navbar-item">
        <div class="buttons">
          <a class="button">
            <Icon :icon="['fas', 'plus']" text="Import" />
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
import ModloaderInstaller from '@/components/modals/ModloaderInstaller.vue'
// eslint-disable-next-line
const props = defineProps<{
  hasSidebar: boolean
}>()
const emit = defineEmits(["sidebar", "update-modpacks"])

let showCreatePack = ref(false)
let pendingModpackInstall = ref<Modpack>()

async function onModpackCreated(pack) {
  await emit('update-modpacks', pack)
  pendingModpackInstall.value = pack
}

const sidebarIcon = computed(() => {
  return [
    'fas',
    // 'bars'
    props.hasSidebar ? 'circle-check' : 'circle'
  ]
})
</script>
