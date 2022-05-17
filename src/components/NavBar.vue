<template>
<nav class="navbar is-black" role="navigation" aria-label="main navigation">
  <div id="navbarBasicExample" class="navbar-menu">
    <div class="navbar-start">
      <a @click="emit('sidebar')" class="navbar-item">
        <span class="icon-text">
          <span class="icon">
            <fa-icon :icon="sidebarIcon" />
          </span>
          <span>Minecraft Mod Manager</span>
        </span>
      </a>

      <div class="navbar-item has-dropdown is-hoverable">
        <a class="navbar-link">
          More
        </a>

        <div class="navbar-dropdown">
          <a class="navbar-item">
            About
          </a>
          <a class="navbar-item">
            Jobs
          </a>
          <a class="navbar-item">
            Contact
          </a>
          <hr class="navbar-divider">
          <a class="navbar-item">
            Report an issue
          </a>
        </div>
      </div>
    </div>

    <div class="navbar-end">
      <div class="navbar-item">
        <div class="buttons">
          <a class="button">
            <Icon :icon="['fas', 'plus']" text="Import" />
          </a>
          <a class="button is-info">
            <Icon :icon="['fas', 'plus']" text="New Modpack" />
          </a>
        </div>
      </div>
    </div>
  </div>
</nav>
</template>

<script setup lang="ts">
import { computed } from 'vue'
// eslint-disable-next-line
const props = defineProps<{
  hasSidebar: boolean
}>()
const emit = defineEmits(["sidebar"])

const sidebarIcon = computed(() => {
  return [
    'fas',
    // 'bars'
    props.hasSidebar ? 'circle-check' : 'circle'
  ]
})
</script>
