<template>
<div class="ml-5 my-2">
  <!-- <Icon :icon="['fa', props.node?.children ? 'folder' : 'cube']" :text="props.node.name" /> -->
  <div class="field" v-if="!props.hidden">
    <input ref="checkbox" class="is-checkradio" :id="props.node.id" type="checkbox" @input="onSelect" :style="'display: none'">
    <label :for="props.node.id" >
      <fa-icon :icon="icon" />
      {{props.node.name}}
    </label>
  </div>
  <ul v-if="isFolder">
    <li v-for="child of children" :key="child.id">
      <TreeNode :parentId="props.node.id" :node="child" @select="onChildSelected" />
    </li>
  </ul>
</div>
</template>

<script setup lang="ts">
import TreeNode from '@/components/TreeNode.vue'
import { computed, ref, onBeforeMount, watch } from 'vue'
const emit = defineEmits(['select'])
const props = defineProps<{
  parentId?: string,
  node: File,
  hidden?: boolean
}>()
let checkbox = ref()
let children = ref(props.node.children)

function onSelect() {
  const selected = checkbox.value.checked
  /*if(children.value) {
    for(let child of children.value) {
      child.selected = selected
    }
  }*/
  emit('select', {
    name: props.node.name,
    selected,
    id: props.node.id
  })
}
/*function recurseDown(c) {
  if(children.value) {
    for(let child of children.value) {
      child.selected = selected
      recurseDown(child)
    }
  }
}*/

function onChildSelected(child) {
  return emit('select', child)

  /*console.log('child was selected, updating parent (me)')
  children.value.find(c => child.name === c.name).selected = child.selected
  if(isFolder.value && !props.hidden) {
    const selected = children.value.reduce((count, child) => {
      if(child.selected) count++
      return count
    }, 0)
    checkbox.value.indeterminate = false
    if(selected === children.value.length) {
      checkbox.value.checked = true
    } else {
      if(selected > 0)
        checkbox.value.indeterminate = true
      checkbox.value.checked = false
    }
  }*/
}

const isFolder = computed(() => props.node.children)
const icon = computed(() => {
  if(isFolder.value) return ['fa', 'folder']
  const ext = props.node.name.split('.').pop();
  switch(ext) {
    case "png":
    case "jpg":
    case "webp":
    case "gif":
      return ['far', 'file-image']
    case "json":
    case "toml":
      return ['far', 'file-code']
    case "txt":
    case "log":
      return ['far', 'file-lines']
    default:
      return ['far', 'file']
  }
})
onBeforeMount(() => {
  if(children.value) {
    children.value = children.value.sort((b, a) => {
      if(a.children) {
        if(b.children) return 1
        else return 0
      }
      return -1
    })
  }
  if(props.node.indeterminate) {
    checkbox.value.indeterminate = true
  }
})
watch(() => props.node.selected, () => {
  checkbox.value.checked = props.node.selected
})
watch(() => props.node.indeterminate, (value) => {
  checkbox.value.indeterminate = value
})
</script>
