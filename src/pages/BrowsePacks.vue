<template>
<div>
  <PackView :modpacks="modpacks" />
</div>
</template>

<script setup lang="ts">
import { Entry } from '../types/External';
import PackView from '@/components/pack/PackView.vue'
import { InstallState } from '../types/Pack';

const modpacks: Entry[] = [
  {
    project: {
      id: "ae2",
      author: "Bob",
      name: "Applied Energistics 2",
      description: "Modern rendering engine and client-side optimization mod for Minecraft",
      icon_url: "https://cdn.modrinth.com/data/AANobbMI/icon.png",
    },
    installState: InstallState.NotInstalled
  },
  {
    project: {
      id: "half-life-2",
      author: "Valve",
      name: "Half Life 3",
      description: "What.",
      icon_url: "https://cdn.modrinth.com/data/AANobbMI/icon.png"
    },
    installState: InstallState.NotInstalled
  },
  {
    project: {
      id: "rdr2",
      author: "Rockstar Guns",
      name: "Red Dead Redemption III",
      description: "Nooooooooooooooooooooo",
      icon_url: "https://cdn.modrinth.com/data/AANobbMI/icon.png"
    },
    installState: InstallState.Installing
  }
]
</script>
