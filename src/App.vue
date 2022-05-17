<template>
<div>
  <NavBar :has-sidebar="hasSidebar" @sidebar="hasSidebar = !hasSidebar" />
  <br>
  <div class="columns">
    <div class="column is-2 ml-1" v-show="hasSidebar">
      <SideBar @view="setView" :view="mainView" />
    </div>
    <div class="column mr-1 ml-1">
      <component :is="activeView" />
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import NavBar from '@/components/NavBar.vue'
import SideBar from '@/components/SideBar.vue'

import MyPacks from '@/pages/MyPacks.vue'
import BrowsePacks from '@/pages/BrowsePacks.vue'
import { View } from '@/types/Pack';

const hasSidebar = ref(true)
const mainView = ref<View>(View.MyPacks)
const activeView = ref<any>(MyPacks)

async function setView(viewId: View) {
  mainView.value = viewId
  switch(viewId) {
    case View.BrowsePacks:
      activeView.value = BrowsePacks
      break
    default: {
      activeView.value = MyPacks
    }
  }
}

</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}
</style>
