<template>
<div class="box" >
  <div class="columns is-mobile">
    <div class="column is-1">
      <figure class="image is-64x64 is-pulled-left">
        <img :src="props.entry.project.icon_url || DefaultModImage" />
      </figure>
    </div>
    <div class="column">
      <h4 class="title is-5">{{props.entry.project.title}} |
        <em class="subtitle has-text-gray is-6" v-if="props.entry.project.author">by {{props.entry.project.author}}</em>
      </h4>
      <p class="subtitle is-6">{{props.entry.project.description}}</p>
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
      <slot name="rightColumn" />
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import DefaultModImage from '@/assets/default_mod.png'
import { Entry } from '../types/External';
const props = defineProps<{
  entry: Entry
}>()
</script>
