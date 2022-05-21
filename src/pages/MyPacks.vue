<template>
<div>
  <h4 class="title is-4" v-if="!selectedPack">Installed Modpacks</h4>
  <PackDetails v-if="selectedPack" :pack="selectedPack" @goback="selectedPack = undefined" />
  <PackList v-else :packs="props.modpacks" @select="(pack) => selectedPack = pack" />
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

watch(() => props.modpacks, () => {
  console.debug('without this, code below doesnt work. help me.')
  if(selectedPack.value) {
    const id = selectedPack.value.id
    selectedPack.value = props.modpacks.find(pack => pack.id === id)
  }
}, { deep: true })

</script>
