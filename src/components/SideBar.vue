<template>
<aside class="menu">
  <p class="menu-label">
    Dashboard
  </p>
  <ul class="menu-list">
    <li><a>My Modpacks</a></li>
  </ul>
  <p class="menu-label">
    Sources
  </p>
  <ul class="menu-list">
    <li><a>CurseForge</a></li>
    <li><a>Modrinth</a></li>
    <!-- <li><a>ModManager</a></li> -->
  </ul>
  <p class="menu-label">
    Misc
  </p>
  <ul class="menu-list">
    <li><a>Settings</a></li>
    <li><a>
      <span class="icon-text">
        <span class="icon">
          <fa-icon :icon="['fab', 'github']" />
        </span>
        <span>Github</span>
      </span>
    </a></li>
  </ul>
</aside>
</template>
