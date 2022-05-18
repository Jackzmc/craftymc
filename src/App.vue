<template>
<div>
  <NavBar :has-sidebar="hasSidebar" @sidebar="hasSidebar = !hasSidebar" />
  <br>
  <div class="columns mt-6">
    <div class="column is-2 ml-1" v-show="hasSidebar">
      <SideBar />
    </div>
    <div class="column mr-1 ml-1">
      <router-view />
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import NavBar from '@/components/NavBar.vue'
import SideBar from '@/components/SideBar.vue'

const hasSidebar = ref(true)
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}
</style>
