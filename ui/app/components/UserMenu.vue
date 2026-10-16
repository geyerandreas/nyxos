<script setup lang="ts">
defineProps<{
  collapsed?: boolean
}>()

const toast = useToast()
const { isLoggingOut, logout } = useAuth()

async function handleSignOut() {
  try {
    await logout()
  } catch {
    toast.add({
      title: 'Unable to sign out',
      description: 'Please try again.',
      color: 'error'
    })
  }
}
</script>

<template>
  <UDropdownMenu
    :items="[[{
      label: 'Sign out',
      icon: 'i-lucide-log-out',
      disabled: isLoggingOut,
      onSelect: handleSignOut
    }]]"
  >
    <UButton
      color="neutral"
      variant="ghost"
      block
      :square="collapsed"
      :loading="isLoggingOut"
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
