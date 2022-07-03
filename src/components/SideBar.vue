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
    <li><a href="https://github.com/Jackzmc/craftymc" target="_blank">
      <Icon :icon="['fab', 'github']" text="Github"
        right-icon-class="has-text-info" :icon-right="['fas', 'arrow-up-right-from-square']"
      />
    </a></li>
    <li><router-link to="/about">
      <Icon :icon="['fa', 'info-circle']" text="About" />
    </router-link></li>
  </ul>
  <template v-if="categories">
    <p class="menu-label  has-text-centered">
      Categories
    </p>
    <ul class="menu-list">
      <li v-for="category in categories" :key="category.name">
        <div class="field pl-2 mt-2">
          <input ref="checkbox" class="is-checkradio" :id="category.name" type="checkbox" :style="'display: none'"
            v-model="selectedCategories" @change="onCategoryChosen" name="category" :value="category.name"
          >
          <label :for="category.name" >
            <span class="custom-icon is-small" v-html="category.icon" />
            &nbsp;{{category.name}}
          </label>
          </div>
      </li>
    </ul>
  </template>
  <!--<ul class="menu-label">
    Ad
  </ul>
  <figure class="image is-9by16">
    <img src="https://via.placeholder.com/1280x960/?text=Ad">
  </figure>-->
</aside>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { getVersion } from '@tauri-apps/api/app'
const emit = defineEmits(["selected"])
const props = defineProps<{
  showData?: object
}>()

const selectedCategories = ref([])
const categories = ref()

watch(() => props.showData, (value) => {
  if(value && value.type === "category-picker") {
    selectedCategories.value = []
    if(!value.for) categories.value = undefined
    else getCategoriesFor(value.for)
  }
}, { deep: true })

async function getCategoriesFor(type = "modpack" | "mod") {
  const appVersion = await getVersion()
  const response = await fetch(`https://api.modrinth.com/v2/tag/category`, {
    headers: {
      'User-Agent': `Jackzmc/CraftyMc v${appVersion}`
    }
  })
  const json = await response.json()
  if(response.ok) {
    categories.value = json.filter(tag => tag.project_type === type)
  } else {
    throw new Error(json.description || json.message || json.error)
  }
}

function onCategoryChosen() {
  emit("selected", selectedCategories.value)
}

</script>

<style>
[class*=" custom-icon"], [class^=custom-icon] {
    display: inline-block;
    width: 1em;
    height: 1em;
    stroke-width: 0;
    stroke: currentColor;
    fill: currentColor;
    line-height: 1;
    position: relative;
    top: -.05em;
    vertical-align: middle;
}
</style>
