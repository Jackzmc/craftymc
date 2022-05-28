<template>
<table class="table is-fullwidth">
  <thead>
    <tr>
      <th>Name</th>
      <th>Author</th>
      <th>File</th>
      <th>Action</th>
      <th>Enabled</th>
    </tr>
  </thead>
  <tbody>
    <tr v-for="mod in props.mods" :key="mod.id">
      <td>{{mod.name}}</td>
      <td>{{mod.author}}</td>
      <td>{{mod.filenames[0]}}</td>
      <td>
        <div class="buttons">
        <!-- <a class="button is-small is-white" data-tooltip="Update"><Icon :icon="['fas', 'download']" /></a> -->
        <a class="button is-small is-white" data-tooltip="Delete"><Icon :icon="['fas', 'trash']" /></a>
        </div>
      </td>
      <td>
        <input id="mod1-active" type="checkbox" name="switchRoundedDefault" class="switch is-rounded is-info" checked="checked">
        <label for="mod1-active"></label>
      </td>
    </tr>
  </tbody>
</table>
</template>

<script setup lang="ts">
import { SavedModEntry } from '../types/Pack';
const props = defineProps<{
  mods: SavedModEntry[],
}>()
</script>
