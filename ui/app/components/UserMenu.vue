<script setup lang="ts">
defineProps<{
  collapsed?: boolean
}>()

const router = useRouter()
</script>

<template>
  <UDropdownMenu
    :items="[[{
      label: 'Sign out',
      icon: 'i-lucide-log-out',
      onSelect: () => router.push('/login')
    }]]"
  >
    <UButton
      color="neutral"
      variant="ghost"
      block
      :square="collapsed"
      class="justify-start"
    >
      <UAvatar
        text="NY"
        size="sm"
      />
      <span
        v-if="!collapsed"
        class="truncate"
      >Account</span>
    </UButton>
  </UDropdownMenu>
</template>
