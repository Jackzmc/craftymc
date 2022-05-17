<template>
<div class="field">
  <label class="label">{{props.label}}</label>
  <div class="control">
    <slot />
  </div>
</div>
</template>

<script setup lang="ts">
const props = defineProps<{label: string}>()
</script>
