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
      <div class="box is-dark" v-for="mod in mods" :key="mod.id" >
        <div class="columns is-mobile">
          <div class="column is-2">
            <figure class="image is-128x128 is-pulled-left">
              <img :src="mod.icon_url || DefaultModImage" />
            </figure>
          </div>
          <div class="column">
            <h4 class="title is-4">{{mod.name}} |
              <em class="subtitle has-text-gray is-6" v-if="mod.author">by {{mod.author}}</em>
            </h4>
            <p class="subtitle is-6">{{mod.description}}</p>
            <br>
            <div class="level">
              <div class="level-left">
                <div class="level-item" data-tooltip="Downloads">
                 <Icon :icon="['fa', 'download']" text="5.2 M" />
                </div>
                <div class="level-item" data-tooltip="Last updated">
                  <Icon :icon="['fa', 'clock']" text="5/17/2022" />
                </div>
                <div class="level-item" data-tooltip="Minecraft version">
                  <Icon :icon="['fa', 'cube']" text="1.16.5" />
                </div>
              </div>
            </div>
          </div>
          <div class="column is-1 mr-5">
            <p v-if="mod.installed">Installed</p>
            <a v-else class="button is-info">Install</a>
          </div>
        </div>
      </div>
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
import { ExternalProject } from '../types/External';
import FilterControls from '@/components/FilterControls.vue'
import DefaultModImage from '@/assets/default_mod.png'
import Field from '@/components/form/Field.vue'
import { Modpack } from '../types/Pack';
const emit = defineEmits(["close"])
const props = defineProps<{
  pack: Modpack
}>()

const mods: ExternalProject[] = [
  {
    id: "ae2",
    author: "Bob",
    name: "Applied Energistics 2",
    description: "Modern rendering engine and client-side optimization mod for Minecraft",
    icon_url: "https://cdn.modrinth.com/data/AANobbMI/icon.png",
    installed: true
  },
  {
    id: "half-life-2",
    author: "Valve",
    name: "Half Life 3",
    description: "What.",
    icon_url: "https://cdn.modrinth.com/data/AANobbMI/icon.png"
  }
]
</script>
