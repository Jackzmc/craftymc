<template>
<div>
  <h4 class="title is-4" v-if="!selectedPack">Installed Modpacks</h4>
  <PackDetails v-if="selectedPack" :pack="selectedPack" @goback="selectedPack = undefined" :edit-selected="editSelected" />
  <PackList v-else :packs="props.modpacks" @select="pack => selectPack(pack)" @edit="pack => selectPack(pack, true)" />
</div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { Modpack } from '@/types/Pack'
import PackList from '@/components/pack/PackList'
import PackDetails from '@/pages/PackDetails'

const props = defineProps<{
  modpacks: Modpack[]
}>()

let selectedPack = ref<Modpack>()
let editSelected = ref(false)

watch(() => props.modpacks, () => {
  console.debug('without this, code below doesnt work. help me.')
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
