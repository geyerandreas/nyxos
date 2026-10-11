<script setup lang="ts">
import { useDashboard } from '../composables/useDashboard'

const { isNotificationsSlideoverOpen } = useDashboard()
</script>

<template>
  <USlideover
    v-model:open="isNotificationsSlideoverOpen"
    title="Notifications"
  >
    <template #body>
      <p class="text-sm text-muted">
        You are all caught up.
      </p>
    </template>
  </USlideover>
</template>
