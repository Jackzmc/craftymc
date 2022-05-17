<template>
<div class="field">
  <label v-if="props.label" class="label">{{props.label}}</label>
  <div :class="controlClass">
    <slot />
    <span class="icon is-small is-left" v-if="props.iconLeft">
      <fa-icon class="is-left" :icon="props.iconLeft" />
    </span>
    <span class="icon is-small is-right" v-if="props.iconRight">
      <fa-icon :icon="props.iconRight" />
    </span>
  </div>
</div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
const props = defineProps<{
  label?: string,
  iconLeft?: string[],
  iconRight?: string[]
}>()

const controlClass = computed(() => {
  const classes = ['control']
  if(props.iconLeft) classes.push('has-icons-left')
  if(props.iconRight) classes.push('has-icons-right')
  return classes
})
</script>
