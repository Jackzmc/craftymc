<template>
<div>
  <div class="level">
    <div class="level-left">
      <div class="level-item">
        <HorizontalField title="Sort by">
          <div class="select">
            <select>
              <option v-for="(display, key) in SORTS" :key="key" :value="key">{{display}}</option>
            </select>
          </div>
        </HorizontalField>
      </div>
      <div class="level-item">
        <HorizontalField title="Filter by">
          <div class="select">
            <select>
              <option v-for="(display, key) in FILTERS" :key="key" :value="key">{{display}}</option>
            </select>
          </div>
        </HorizontalField>
      </div>
    </div>
    <div class="level-right">
      <div class="level-item">
        <input class="slider is-fullwidth" step="1" min="0" max="100" value="50" type="range">
      </div>
    </div>
  </div>
  <div class="columns is-multiline">
    <div class="column" v-for="pack of PACKS" :key="pack.name">
      <Pack :pack="pack" />
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
// import {  } from 'vue'
import HorizontalField from '@/components/HorizontalField.vue'
import { Modpack } from '@/types/Pack'
import Pack from '@/components/Pack'

const FILTERS = {
  all: "All Modpacks"
}

const SORTS = {
  recentlyPlayed: "Recently Played",
  mostPlayed: "Most Played",
  name: "Name",
  mcVersion: "Game Version",
  created: "Creation Date"
}

const PACKS: Modpack[] = [
  {
    name: "Direwolf20",
    imageUrl: "https://cdn-raw.modrinth.com//data/KmiWHzQ4/62f2e3a020f761f275f0a7f3ebca3327576edb4c.png",
    author: "Etho",
    versions: {
      minecraft: "1.16.5",
      forge: "1.16.5",
      pack: "1.0.0"
    }
  },
  {
    name: "Direwolf20",
    imageUrl: "https://bulma.io/images/placeholders/1280x960.png",
    author: "Etho",
    versions: {
      minecraft: "1.16.5",
      forge: "1.16.5"
    }
  },
  {
    name: "Direwolf20",
    imageUrl: "https://bulma.io/images/placeholders/1280x960.png",
    author: "FTB",
    versions: {
      minecraft: "1.16.5",
      forge: "1.16.5"
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
    }
  }
]

</script>
