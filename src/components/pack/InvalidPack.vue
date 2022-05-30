<template>
<div class="card" @click="showReason">
  <a class="card-image">
    <figure class="image is-4by3">
      <img :src="InvalidPackImage" alt="Invalid modpack">
    </figure>
  </a>
  <div class="card-content">
    <div class="media">
      <div class="media-content">
        <p class="has-text-danger"><b>{{props.name}}</b></p>
      </div>
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import InvalidPackImage from '@/assets/invalid_pack.png'
const props = defineProps<{
  name: string,
  reason: string
}>()

function showReason() {
  alert("Modpack is invalid:\n" + props.reason)
}
</script>
