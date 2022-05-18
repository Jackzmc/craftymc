<template>
<div>
  <div class="columns">
    <div class="column is-10">
      <h4 class="title is-4">Find a modpack</h4>
      <Field :icon-left="['fa', 'search']">
        <!-- TODO: Add 'x' to right on search -->
        <input type="text" class="input" placeholder="Search for a modpack" />
      </Field>
    </div>
  </div>
  <FilterControls />
  <EntryCard v-for="entry in props.modpacks" :entry="entry" :key="entry.project.id">
    <template v-slot:rightColumn>
        <a
          :disabled="entry.installState != InstallState.NotInstalled ? true : undefined"
          :class="['button', 'is-info', {'is-loading': entry.installState == InstallState.Installing}]"
        >Install</a>
    </template>
  </EntryCard>
</div>
</template>

<script setup lang="ts">
import { Entry } from '@/types/External';
import FilterControls from '@/components/FilterControls.vue'
import EntryCard from '@/components/EntryCard.vue'
import Field from '@/components/form/Field.vue'
import { InstallState } from '@/types/Pack';

const props = defineProps<{
  modpacks: Entry[]
}>()
</script>
