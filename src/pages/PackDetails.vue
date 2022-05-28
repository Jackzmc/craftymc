<template>
<div>
  <component :is="component" :pack="props.pack" v-if="component" @close="showSubview(Subview.None)" @change-modloader="changeModloader"/>
  <template v-if="!hideSelf"> <!-- a little hacky -->
  <a class="button mb-2">
    <Icon :icon="['fas', 'arrow-left']" text="Back" @click="emit('goback')" />
  </a>
  <div class="box has-background-dark">
    <div class="columns is-mobile is-centered is-vcentered">
      <div class="column is-2">
        <figure class="image is-128x128 is-pulled-left">
          <img :style="'width:128px;height:128px'" :src="props.pack.imageUrl" />
        </figure>
      </div>
      <div class="column">
        <h4 class="title is-4 has-text-white">{{props.pack.name}}
          <em class="subtitle has-text-light is-6" v-if="props.pack.author">| by {{props.pack.author}}</em>
        </h4>
        <br>
        <div class="level">
          <div class="level-left">
            <div class="level-item" data-tooltip="Last played">
              <Icon :icon="['fas', 'play']" icon-class="has-text-white" :text="formatRelative(props.pack.lastPlayed)" />
            </div>
            <div class="level-item" data-tooltip="Game version">
              <Icon :icon="['fas', 'cube']" icon-class="has-text-white" :text="props.pack.versions.minecraft" />
            </div>
            <div class="level-item" data-tooltip="Mod loader">
              <div class="icon-text has-text-white" >
                <Icon :icon="['fa', 'hammer']" icon-class="has-text-white" :text="props.pack.settings.modloaderType + '-' + props.pack.versions.modloader" />
              </div>
            </div>
            <div class="level-item" data-tooltip="Pack version" v-if="props.pack.versions.pack">
              <div class="icon-text has-text-white" >
                <span>{{props.pack.versions.pack}}</span>
              </div>
            </div>
          </div>
          <div class="level-right">
            <div class="level-item">
              <div class="buttons">
                <a class="button is-dark" @click="event => contextMenu.open(event)">
                  <Icon :icon="['fas', 'ellipsis-vertical']" />
                </a>
                <a class="button is-success" @click="showSubview(Subview.AddContent)">
                  <Icon :icon="['fas', 'plus']" text="Add Content" />
                </a>
                <a class="button is-info" :style="'width: 6em'" @click="launch">Play</a>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <Tabs inner-wrapper-class="tabs" :options="{useUrlFragment:false, defaultTabHash: 'mods'}">
    <Tab name='debug'>
      <pre>{{ JSON.stringify(props.pack, null, 2) }}</pre>
    </Tab>
    <Tab name="Overview">

    </Tab>
    <Tab name="Mods" id="mods">
      <Modlist :mods="pack.mods" />
    </Tab>
    <Tab name="Versions">

    </Tab>
  </Tabs>
  </template>
  <ContextMenu class="box" ref="contextMenu">
    <aside class="menu">
      <ul class="menu-list">
        <li><a class="has-text-info" @click="launch()">
          <Icon :icon="['fa', 'play']" text="Play" />
        </a></li>
        <li><a @click="showSubview(Subview.SettingsModal)">
          <Icon :icon="['fa', 'cog']" text="Settings" />
        </a></li>
        <li><a @click="invoke('open_modpack_folder', { packId: props.pack.id })">
          <Icon :icon="['fa', 'folder']" text="Open Folder" />
        </a></li>
        <li><a @click="showSubview(Subview.Export)">
          <Icon :icon="['fa', 'file-export']" text="Export" />
        </a></li>
        <li><a class="has-text-danger"  @click="showSubview(Subview.DeletePack)">
          <Icon :icon="['fa', 'trash']" text="Delete" />
        </a></li>
      </ul>
    </aside>
  </ContextMenu>
</div>
</template>

<script setup lang="ts">
import { Tabs, Tab } from 'vue3-tabs-component'
import { Modpack } from '@/types/Pack'
import { ref, defineAsyncComponent, markRaw, onMounted } from 'vue'
import Modlist from '@/components/Modlist.vue'
import ContextMenu from '@/components/ContextMenu.vue'
import { invoke} from '@tauri-apps/api/tauri'

let contextMenu = ref()
let component = ref()
let hideSelf = ref(false)

const emit = defineEmits(["goback", "change-modloader"])
const props = defineProps<{
  pack: Modpack,
  editSelected: boolean
}>()

const enum Subview {
  None = 0,
  SettingsModal = 1,
  AddContent = 2,
  DeletePack = 3,
  Export = 4
}

function formatRelative(value: number, locale?: string) {
  if(!value) return "Never"
  const date = new Date(value);
  const deltaDays = (date.getTime() - Date.now()) / (1000 * 3600 * 24);
  if(deltaDays <= 0) return "Today"
  else if(deltaDays <= 1) return "Yesterday"
  const formatter = new Intl.RelativeTimeFormat(locale);
  return formatter.format(Math.round(deltaDays), 'days');
}

async function launch() {
  await invoke('launch_modpack', { id: props.pack.id })
}

async function showSubview(subview: Subview) {
  hideSelf.value = false
  switch(subview) {
    case Subview.None:
      component.value = undefined
      if(props.editSelected)
        emit('goback')
      break
    case Subview.SettingsModal:
      component.value = markRaw(defineAsyncComponent(() => import('@/components/modals/PackSettingsModal.vue')))
      break
    case Subview.AddContent:
      hideSelf.value = true
      component.value = markRaw(defineAsyncComponent(() => import('@/pages/AddContent.vue')))
      break
    case Subview.Export:
      component.value = markRaw(defineAsyncComponent(() => import('@/components/modals/ExportModal.vue')))
      break
    case Subview.DeletePack:
      component.value = markRaw(defineAsyncComponent(() => import('@/components/modals/DeletePackConfirmModal.vue')))
      break
  }
  contextMenu.value.close()
}

function changeModloader(pack) {
  emit('change-modloader', pack)
  showSubview(Subview.None)
}

onMounted(async() => {
  if(props.editSelected) await showSubview(Subview.SettingsModal)
})
</script>
