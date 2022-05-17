<template>
<div class="field is-horizontal">
  <div class="field-label is-normal">
    <label class="label" style="white-space: nowrap">{{props.title}}</label>
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
import { defineProps } from 'vue'

const props = defineProps<{
  title: string
}>()
</script>
