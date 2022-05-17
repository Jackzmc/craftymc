<template>
<div>
  <PackDetails v-if="selectedPack" :pack="selectedPack" @goback="selectedPack = undefined" />
  <PackList v-else :packs="packs" @select="(pack) => selectedPack = pack" />
</div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Modpack } from '@/types/Pack'
import PackList from '@/components/pack/PackList'
import PackDetails from '@/pages/PackDetails'

const packs: Modpack[] = [
  {
    name: "Direwolf20",
    imageUrl: "https://cdn-raw.modrinth.com//data/KmiWHzQ4/62f2e3a020f761f275f0a7f3ebca3327576edb4c.png",
    author: "Etho",
    versions: {
      minecraft: "1.16.5",
      forge: "1.16.5",
      pack: "1.0.0"
    },
    settings: {
      javaMemory: 5000
    }
  },
  {
    name: "Direwolf20",
    author: "Etho",
    versions: {
      minecraft: "1.16.5",
      forge: "1.16.5"
    },
    settings: {

    }
  },
  {
    name: "Direwolf20",
    author: "FTB",
    versions: {
      minecraft: "1.16.5",
      forge: "1.16.5"
    },
    settings: {

    }
  },
  {
    name: "Direwolf20",
    imageUrl: "https://cdn-raw.modrinth.com//data/KmiWHzQ4/62f2e3a020f761f275f0a7f3ebca3327576edb4c.png",
    author: "Etho",
    versions: {
      minecraft: "1.16.5",
      forge: "1.16.5",
      pack: "1.0.0"
    },
    settings: {

    }
  }
]

let selectedPack = ref<Modpack>()

</script>
