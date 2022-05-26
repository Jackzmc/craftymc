<template>
<FileTree
  :items="files"
  :is-checkable="true"
  :hide-guide-lines="false"
  v-model:checkedItems="selectedItems"
>
  <template v-slot:item-prepend-icon="treeViewItem" >
  <Icon :icon="['fa', 'folder']" v-if="treeViewItem.type === 'folder'" />
  <Icon :icon="['fa', 'cube']" v-else />
        <!-- <img src="./assets/folder.svg"
              alt="folder"
              v-if="treeViewItem.type === 'folder'"
              height="20" width="20" /> -->
    folder
  </template>
</FileTree>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import FileTree from 'vue3-tree-vue'


const selectedItems = ref<TreeViewItem[]>([]);


//eslint-disable-next-line
/*const props = defineProps<{
  files: File[]
}>()*/
</script>
