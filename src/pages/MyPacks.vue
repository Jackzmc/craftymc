<template>
<div>
  <h4 class="title is-4" v-if="!selectedPack">Installed Modpacks</h4>
  <PackDetails v-if="selectedPack" :pack="selectedPack"
    @goback="selectedPack = undefined" :edit-selected="editSelected" @change-modloader="v => emit('change-modloader', v)"
    @show="value => emit('show', value)"
    :selected="props.selected"
  />
  <PackList v-else :packs="props.modpacks" @select="pack => selectPack(pack)" @edit="pack => selectPack(pack, true)" />
</div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { Modpack } from '@/types/Pack'
import PackList from '@/components/pack/PackList'
import PackDetails from '@/pages/PackDetails'

const emit = defineEmits(["change-modloader", "show"])

const props = defineProps<{
  modpacks: Modpack[],
  selected?: any
}>()

let selectedPack = ref<Modpack>()
let editSelected = ref(false)

watch(() => props.modpacks, () => {
  console.debug('(vue3 watcher weirdness requires this console log. I don\'t know why. It scares me.)')
  if(selectedPack.value) {
    const id = selectedPack.value.id
    selectedPack.value = props.modpacks.find(pack => pack.id === id)
  }
}, { deep: true })

function selectPack(pack: Modpack, edit = false) {
  selectedPack.value = pack
  editSelected.value = edit
}

</script>
