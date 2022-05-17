<template>
<div>
  <PackSettingsModal :pack="pack" v-if="settingsModalActive" @close="settingsModalActive=false" />
  <a class="button mb-2">
    <Icon :icon="['fas', 'arrow-left']" text="Back" @click="emit('goback')" />
  </a>
  <div class="box has-background-dark">
    <div class="columns is-mobile is-centered is-vcentered">
      <div class="column is-2">
        <figure class="image is-128x128 is-pulled-left">
          <img :src="props.pack.imageUrl" />
        </figure>
      </div>
      <div class="column">
        <h4 class="title is-4 has-text-white">{{props.pack.name}}
          <em class="subtitle has-text-light is-6" v-if="props.pack.author">by {{props.pack.author}}</em>
        </h4>
        <br>
        <div class="level">
          <div class="level-left">
            <div class="level-item" data-tooltip="Last played">
              <Icon :icon="['fas', 'play']" icon-class="has-text-white" text="Yesterday" />
            </div>
            <div class="level-item" data-tooltip="Game version">
              <Icon :icon="['fas', 'cube']" icon-class="has-text-white" :text="props.pack.versions.minecraft" />
            </div>
            <div class="level-item" data-tooltip="Pack version" v-if="props.pack.versions.pack">
              <div class="icon-text has-text-white" >
                <span>{{props.pack.versions.pack}}</span>
              </div>
            </div>
          </div>
          <div class="level-right">
            <div class="level-item">
              <div class="buttons">
                <a class="button is-dark">
                  <Icon :icon="['fas', 'cog']" @click="settingsModalActive = true" />
                </a>
                <a class="button is-info" style="width: 6em">Play</a>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <Tabs inner-wrapper-class="tabs" :options="{useUrlFragment:false}">
    <Tab name="Overview">

    </Tab>
    <Tab name="Mods">

    </Tab>
    <Tab name="Versions">

    </Tab>
  </Tabs>
</div>
</template>

<script setup lang="ts">
import { Tabs, Tab } from 'vue3-tabs-component'
import { Modpack } from '@/types/Pack'
import { ref } from 'vue'
import PackSettingsModal from '@/components/modals/PackSettingsModal.vue'

const emit = defineEmits(["goback"])
const props = defineProps<{
  pack: Modpack
}>()

let settingsModalActive = ref(false)
</script>
