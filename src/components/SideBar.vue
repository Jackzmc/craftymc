<template>
<aside class="menu">
  <p class="menu-label  has-text-centered">
    Dashboard
  </p>
  <ul class="menu-list">
    <li><router-link to="/" class="icon-text">
      <span class="icon">
        <fa-icon :icon="['fas', 'home']" />
      </span>
      <span>My Modpacks</span>
    </router-link></li>
    <li><router-link to="/browse" class="icon-text">
      <span class="icon">
        <fa-icon :icon="['fas', 'search']" />
      </span>
      <span>Find Modpacks</span>
    </router-link></li>
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
    <li><router-link to="/settings">
      <Icon :icon="['fas', 'sliders']" text="Settings" />
    </router-link></li>
    <li><a>
      <Icon :icon="['fab', 'github']" text="Github"
        right-icon-class="has-text-info" :icon-right="['fas', 'arrow-up-right-from-square']"
      />
    </a></li>
  </ul>
</aside>
</template>

<script setup lang="ts">
</script>
