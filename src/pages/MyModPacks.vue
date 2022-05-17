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
      <div class="card">
        <div class="card-image">
          <figure class="image is-4by3">
            <img src="https://bulma.io/images/placeholders/1280x960.png" alt="Placeholder image">
          </figure>
        </div>
        <div class="card-content">
          <div class="media">
            <div class="media-content">
              <p class="title is-5">{{pack.name}}</p>
              <p class="subtitle is-6">by {{pack.author}}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import HorizontalField from '@/components/HorizontalField.vue'

interface Pack {
  name?: string,
  versions: {
    minecraft?: string,
    forge?: string
  }
}

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

const PACKS: Pack[] = [
  {
    name: "Direwolf20",
    author: "Etho",
    versions: {
      minecraft: "1.16.5",
      forge: "1.16.5"
    }
  },
  {
    name: "Direwolf20",
    author: "Etho",
    versions: {
      minecraft: "1.16.5",
      forge: "1.16.5"
    }
  },
  {
    name: "Direwolf20",
    author: "FTB",
    versions: {
      minecraft: "1.16.5",
      forge: "1.16.5"
    }
  }
]
</script>
