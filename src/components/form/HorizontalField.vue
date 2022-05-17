<template>
<div class="field is-horizontal">
  <div class="field-label is-normal">
    <label class="label" style="white-space: nowrap">{{props.label}}</label>
  </div>
  <div class="field-body">
    <div class="field">
      <div class="control">
        <slot></slot>
      </div>
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
const props = defineProps<{
  label: string
}>()
</script>
