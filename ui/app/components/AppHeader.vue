<script setup lang="ts">
const { isAuthenticated, isLoggingOut, logout, refreshAuthStatus } = useAuth()

onMounted(() => {
  refreshAuthStatus()
})
</script>

<template>
  <header class="border-default bg-default/95 sticky top-0 z-20 border-b backdrop-blur">
    <div class="mx-auto flex h-16 w-full max-w-[1600px] items-center justify-between px-4 sm:px-6 lg:px-8">
      <NuxtLink
        to="/"
        class="flex items-center gap-2 font-semibold tracking-tight text-highlighted"
      >
        <span class="flex size-8 items-center justify-center rounded-lg bg-primary text-white">
          <UIcon
            name="i-lucide-box"
            class="size-4"
          />
        </span>
        <span>Nyxos</span>
      </NuxtLink>

      <UButton
        v-if="isAuthenticated === true"
        color="neutral"
        variant="outline"
        icon="i-lucide-log-out"
        label="Log out"
        :loading="isLoggingOut"
        @click="logout"
      />
      <UButton
        v-else
        color="primary"
        icon="i-lucide-log-in"
        label="Log in"
        to="/login"
      />
    </div>
  </header>
</template>
