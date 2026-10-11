<script setup lang="ts">
defineProps<{
  collapsed?: boolean
}>()
</script>

<template>
  <UButton
    color="neutral"
    variant="ghost"
    block
    :square="collapsed"
    class="justify-start"
  >
    <UIcon
      name="i-lucide-server"
      class="size-5 shrink-0"
    />
    <span
      v-if="!collapsed"
      class="truncate"
    >Nyxos</span>
  </UButton>
</template>
