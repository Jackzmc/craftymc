<template>
<aside class="menu" :style="'position: fixed'">
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
  <!--<ul class="menu-list">
    <li><a>CurseForge</a></li>
    <li><a>Modrinth</a></li>
    <li><a>ModManager</a></li>
  </ul>-->
  <p class="menu-label  has-text-centered">
    Misc
  </p>
  <ul class="menu-list">
    <li><router-link to="/settings">
      <Icon :icon="['fas', 'sliders']" text="Settings" />
    </router-link></li>
    <li><a href="https://github.com/Jackzmc/mc-mod-downloader" target="_blank">
      <Icon :icon="['fab', 'github']" text="Github"
        right-icon-class="has-text-info" :icon-right="['fas', 'arrow-up-right-from-square']"
      />
    </a></li>
    <li><router-link to="/about">
      <Icon :icon="['fa', 'info-circle']" text="About" />
    </router-link></li>
  </ul>
  <p class="menu-label  has-text-centered">
    Categories
  </p>
  <ul class="menu-list">
    <li v-for="category in categories" :key="category">

    </li>
  </ul>
  <!--<ul class="menu-label">
    Ad
  </ul>
  <figure class="image is-9by16">
    <img src="https://via.placeholder.com/1280x960/?text=Ad">
  </figure>-->
</aside>
</template>

<script setup lang="ts">
</script>
