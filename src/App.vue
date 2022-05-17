<template>
<div>
  <NavBar />
  <br>
  <div class="columns">
    <div class="column is-2 ml-1">
      <SideBar />
    </div>
    <div class="column mr-1">
      <MyModPacks />
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import NavBar from '@/components/NavBar.vue'
import SideBar from '@/components/SideBar.vue'
import MyModPacks from '@/pages/MyModPacks.vue'
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}
</style>
