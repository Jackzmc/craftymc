<template>
<div class="card" @mouseover="setActive(true)" @mouseleave="setActive(false)">
  <a class="card-image" @click="emit('select', pack)">
    <figure class="image is-4by3">
      <img :src="props.pack.imageUrl" alt="Modpack image">
      <span class="is-overlay pr-2 py-2">
        <div class="tags has-addons is-pulled-right ml-2"  data-tooltip="Game version">
          <span class="tag">
            <fa-icon :icon="['fa', 'cube']" />
          </span>
          <span class="tag is-black">{{props.pack.versions.minecraft}}</span>
        </div>
        <div class="tags has-addons is-pulled-right" data-tooltip="Pack version" v-if="props.pack.versions.pack">
          <span class="tag is-black">{{props.pack.versions.pack}}</span>
        </div>
      </span>
    </figure>
  </a>
  <div class="card-content">
    <div class="media">
      <div class="media-content" v-if="active">
        <p class="button is-info is-pulled-right is-fullwidth" @click="launch">Play</p>
      </div>
      <div class="media-content" v-else>
        <p><b>{{props.pack.name}}</b></p>
        <p v-if="props.pack.author">by {{props.pack.author}}</p>
      </div>
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Modpack } from '@/types/Pack'
import { invoke } from '@tauri-apps/api/tauri'

const emit = defineEmits(["select"])
const props = defineProps<{
  pack: Modpack
}>()

const active = ref()

function setActive(state: boolean) {
  active.value = state
}

async function launch() {
  // TODO: Show that launcher is running
  const exitCode = await invoke('launch_modpack', { id: props.pack.id })
  console.info('launched modpack exited with code', exitCode)
}
</script>
