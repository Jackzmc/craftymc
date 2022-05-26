<template>
<BaseModal active show-header title="Export Modpack">
  <!-- {{ props.pack.id }} -->
  <TreeNode v-if="rootFile" :node="rootFile" hidden @select="updateRoot" />
  <template v-slot:footer>
    <div class="buttons">
      <a class="button is-info" @click="save">Export</a>
      <a class="button" @click="emit('close')">Cancel</a>
    </div>
  </template>
</BaseModal>
</template>

<script setup lang="ts">
/* eslint-disable */
import BaseModal from './BaseModal.vue'
import TreeNode from '@/components/TreeNode.vue'
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
const emit = defineEmits(['close'])

interface Node {
  id: string,
  name: string,
  selected?: boolean,
  indeterminate?: boolean
  children?: Node[]
}
let rootFile = ref<Node>()

const props = defineProps<{
  pack: Modpack
}>()

function save() {
  const selected = getSelected(rootFile.value)
  console.log('selected', selected)
  invoke('export_modpack', {
    packId: props.pack.id,
    fileName: props.pack.name + ".zip",
    paths: selected
  })
}

function getSelected(root: Node, prefix = "") {
  const selected = []
  /*
  [mymodpack]: [
    folderA: {},
    file
  ]

  */
  for(const child of root.children) {
    if(child.selected) {
      console.log(child.name, prefix)
      selected.push(`${prefix}/${child.name}`)
    }
    if(child.children) {
      selected.push(...getSelected(child, `${prefix}/${child.name}`))
    }
  }
  return selected
}

function updateRoot(updateChild) {
  console.log('root update', updateChild)
  _recurseDown(null, rootFile.value, (parent, node) => {
    if(node.id === updateChild.id) {
      node.selected = updateChild.selected
      console.log('found', node)
      _recurseUp(node, (parent) => {
        console.log('traversing up', parent)
        parent.indeterminate = true
        return true
      })
      if(node.children) {
        _recurseDown(null, node, (parent, child) => {
          child.selected = updateChild.selected
          return true
        })
      }

      return false
    }
    return true
  })
  // _recurse_down(rootFile.value, updateChild.selected, updateChild.id)
  // rootFile.children.value.find(c => child.name === c.name).selected = child.selected
}

/*function _recurse_down(parent, node, selected: boolean, searchKey?: string) {
  for(const child of node.children) {
    if(!searchKey || child.id === searchKey) {
      child.selected = selected
      if(child.children) {
        _recurse_down(parent, child, selected)
      }
      console.log('found', searchKey, selected)
    }else if(child.children) {
      _recurse_down(parent, node, selected, searchKey)
    }
  }
}*/

function _recurseDown(parent: Node, node: Node, func: Function) {
  for(const child of node.children) {
    if(!func(parent, child)) break
    if(child.children) {
      _recurseDown(node, child, func)
    }
  }
}

function _recurseUp(root: Node, func: Function) {
  console.log('traverse', root)
  if(root.parent) {
    return _recurseUp(root.parent, func)
  }
  console.log('hit top of tree')
}

onMounted(async() => {
  let root = await invoke('get_instance_tree', { packId: props.pack.id })
  console.log('root', root)
  root.children = root.children
    .sort((b, a) => {
      if(a.children) {
        if(b.children) return 1
        else return 0
      }
      return -1
    })
  _recurseDown(null, root, (parent, node) => {
    node.parent = parent
    node.indeterminate = false
    if(node.id == "manifest.json") {
      node.selected = true
      node.readOnly = true
    }
    return true
  })
  rootFile.value = root
})
</script>
