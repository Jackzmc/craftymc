<template>
<div :class="'icon-text ' + (props.iconClass || '')">
  <span class="icon">
    <fa-icon :icon="props.icon" :size="props.size" />
  </span>
  <span v-if="props.text">{{props.text}}</span>
  <span class="icon" v-if="iconRight">
    <fa-icon :class="rightIconClass" :icon="props.iconRight" :size="props.size" />
  </span>
</div>
</template>

<script setup lang="ts">
const props = defineProps<{
  icon: string[],
  iconClass?: string
  iconRight?: string[]
  rightIconClass?: string
  text?: string,
  size?: string
}>()
</script>
