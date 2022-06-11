<template>
	<div class="container" ref="container" :style="{
		'display': isOpen ? 'block' : 'none',
		'left': `${position.x}px`,
		'top': `${position.y}px`,
	}">
		<slot :ctx="ctx" />
	</div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

let container = ref()
let isOpen = ref(false)
let position = ref({
	x: 0,
	y: 0
})

let ctx = ref()

function open(event, _ctx) {
	if (event) {
		position.value.x = event.pageX;
		position.value.y = event.pageY;
	}
  ctx.value = _ctx
	isOpen.value = true;
}

function close() {
	isOpen.value = false;
}

defineExpose({
	open,
	close
})

function onClick(e) {
  if (!container.value || !isOpen.value) return;

  const insideMenu = container.value.contains(e.target);

  if (!insideMenu) isOpen.value = false;
}

onMounted(() => {
	document.addEventListener('click', onClick);
})
onUnmounted(() => {
  document.removeEventListener('click', onClick)
})
</script>

<style scoped>
.container {
	position: absolute;
}
</style>
