<template>
<aside class="menu">
  <p class="menu-label  has-text-centered">
    Dashboard
  </p>
  <ul class="menu-list">
    <li><a :class="['icon-text', {'is-active': props.view === View.MyPacks }]" @click="setView(View.MyPacks)">
      <span class="icon">
        <fa-icon :icon="['fas', 'home']" />
      </span>
      <span>My Modpacks</span>
    </a></li>
    <li><a :class="['icon-text', {'is-active': props.view === View.BrowsePacks }]"  @click="setView(View.BrowsePacks)">
      <span class="icon">
        <fa-icon :icon="['fas', 'search']" />
      </span>
      <span>Find Modpacks</span>
    </a></li>
  </ul>
  <p class="menu-label  has-text-centered">
    Sources
  </p>
  <ul class="menu-list">
    <li><a>CurseForge</a></li>
    <li><a>Modrinth</a></li>
    <!-- <li><a>ModManager</a></li> -->
  </ul>
  <p class="menu-label  has-text-centered">
    Misc
  </p>
  <ul class="menu-list">
    <li><a>
      <span class="icon-text">
        <span class="icon">
          <fa-icon :icon="['fas', 'sliders']" />
        </span>
        <span>Settings</span>
      </span>
    </a></li>
    <li><a>
      <span class="icon-text">
        <span class="icon">
          <fa-icon :icon="['fab', 'github']" />
        </span>
        <span>Github</span>
        <span class="icon">
          <fa-icon class="has-text-info" :icon="['fas', 'arrow-up-right-from-square']" />
        </span>
      </span>
    </a></li>
  </ul>
</aside>
</template>

<script setup lang="ts">
// TODO: Rewrite this as like a really shitty vue-router
import { defineEmits, defineProps } from 'vue'
import { View } from '@/types/Pack'
const emit = defineEmits(['view'])

const props = defineProps<{
  view: View
}>()

function setView(view: View) {
  emit('view', view)
}
</script>
