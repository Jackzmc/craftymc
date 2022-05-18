<template>
<div>
  <div class="columns">
    <div class="column is-10">
      <h4 class="title is-4">{{props.pack.name}}</h4>
      <Field :icon-left="['fa', 'search']">
        <!-- TODO: Add 'x' to right on search -->
        <input type="text" class="input" placeholder="Search for mods" />
      </Field>
    </div>
    <div class="column">
      <a class="button is-white is-circular is-pulled-right" @click="emit('close')">
        <fa-icon :icon="['fas', 'times-circle']" size="2x" />
      </a>
    </div>
  </div>
  <Tabs inner-wrapper-class="tabs" :options="{useUrlFragment:false}">
    <Tab name="Mods">
      <FilterControls />
      <EntryCard v-for="entry in mods" :entry="entry" :key="entry.project.id">
      <template v-slot:rightColumn>
          <p v-if="props.entry.installState == InstallState.Installed">Installed</p>
          <a v-else
            :disabled="props.entry.installState != InstallState.NotInstalled ? true : undefined"
            :class="['button', 'is-info', {'is-loading': props.entry.installState == InstallState.Installing}]"
          >Install</a>
      </template>
    </EntryCard>
    </Tab>
    <Tab name="Resource Packs">

    </Tab>

    <Tab name="Maps">

    </Tab>
  </Tabs>
</div>
</template>

<script setup lang="ts">
import { Tab, Tabs } from 'vue3-tabs-component'
import { Entry } from '../types/External';
import FilterControls from '@/components/FilterControls.vue'
import EntryCard from '@/components/EntryCard.vue'
import Field from '@/components/form/Field.vue'
import { Modpack, InstallState } from '../types/Pack';
const emit = defineEmits(["close"])
const props = defineProps<{
  pack: Modpack
}>()

const mods: Entry[] = [
  {
    project: {
      id: "ae2",
      author: "Bob",
      name: "Applied Energistics 2",
      description: "Modern rendering engine and client-side optimization mod for Minecraft",
      icon_url: "https://cdn.modrinth.com/data/AANobbMI/icon.png",
    },
    installState: InstallState.Installed
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
