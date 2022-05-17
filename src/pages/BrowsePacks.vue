<template>
<div>
  <PackList :packs="packs" />
</div>
</template>

<script setup lang="ts">
// import {  } from 'vue'
import { Modpack } from '@/types/Pack'
import PackList from '@/components/PackList'

const packs: Modpack[] = []

</script>
