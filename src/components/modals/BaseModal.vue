<template>
<div :class="['modal', {'is-active': props.active}]">
  <div class="modal-background"></div>
  <div class="modal-card">
    <header class="modal-card-head" v-if="props.showHeader">
      <p class="modal-card-title">{{props.title}}</p>
      <button v-if="props.closable" class="delete" aria-label="close" @click="emit('close')"></button>
    </header>
    <section class="modal-card-body">
      <slot />
    </section>
    <footer class="modal-card-foot" v-if="!hideFooter">
      <slot name="footer">
        <button class="button" @click="emit('close')">Close</button>
      </slot>
    </footer>
  </div>
</div>
</template>

<script setup lang="ts">
const emit = defineEmits(["close"])
const props = defineProps<{
  title?: string,
  closable?: boolean,
  active?: boolean,
  showHeader?: boolean,
  hideFooter?: boolean
}>()
</script>
