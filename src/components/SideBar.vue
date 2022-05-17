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
      <span class="icon-text">
        <span class="icon">
          <fa-icon :icon="['fas', 'sliders']" />
        </span>
        <span>Settings</span>
      </span>
    </router-link></li>
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
</script>
