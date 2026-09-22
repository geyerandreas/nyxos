<script setup lang="ts">
import { ref } from 'vue'

const isUploadOpen = ref(false)

const navigationLinks = [
  { label: 'Overview', icon: 'i-heroicons-home', to: '/dashboard' },
  { label: 'My Packages', icon: 'i-heroicons-squares-2x2', to: '/dashboard/packages' },
  { label: 'API Tokens', icon: 'i-heroicons-key', to: '/dashboard/tokens' },
  { label: 'Documentation', icon: 'i-heroicons-book-open', to: '/dashboard/docs' },
  { label: 'Settings', icon: 'i-heroicons-cog-6-tooth', to: '/dashboard/settings' }
]

const handleLogout = () => {
  useRouter().push('/login')
}
</script>

<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900 flex">
    <!-- Sidebar / Navigation Links -->
    <aside
      class="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 hidden md:flex flex-col justify-between p-4 fixed h-full"
    >
      <div class="space-y-6">
        <div class="flex items-center gap-2 px-2 py-1">
          <UIcon
            name="i-lucide-server"
            class="w-6 h-6 text-primary-500"
          />
          <span class="font-bold text-lg text-gray-900 dark:text-white">PrivatePyPI</span>
        </div>

        <UVerticalNavigation :links="navigationLinks" />
      </div>

      <!-- User Profile Summary Footer -->
      <div
        class="border-t border-gray-200 dark:border-gray-700 pt-4 flex items-center justify-between"
      >
        <div class="flex items-center gap-3">
          <UAvatar
            alt="User Avatar"
            size="sm"
            text="DU"
          />
          <div class="text-sm">
            <p class="font-medium text-gray-700 dark:text-gray-200">
              DevUser
            </p>
            <p class="text-xs text-gray-500">
              Token active
            </p>
          </div>
        </div>
        <UButton
          color="neutral"
          variant="ghost"
          icon="i-lucide-log-out"
          @click="handleLogout"
        />
      </div>
    </aside>

    <!-- Main Content Area -->
    <div class="flex-1 md:pl-64 flex flex-col min-h-screen">
      <!-- Top Navbar -->
      <header
        class="h-16 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between px-6 sticky top-0 z-10"
      >
        <div class="flex items-center gap-4">
          <h1 class="text-xl font-semibold text-gray-800 dark:text-white">
            Dashboard
          </h1>
        </div>
        <div class="flex items-center gap-4">
          <UColorModeButton />
          <UButton
            label="Upload Package"
            icon="i-lucide-cloud-upload"
            color="primary"
            @click="isUploadOpen = true"
          />
        </div>
      </header>

      <!-- Page Target View -->
      <main class="p-6 flex-1 max-w-7xl w-full mx-auto">
        <slot />
      </main>
    </div>

    <!-- Reusable Upload Modal triggered from header -->
    <UModal v-model="isUploadOpen">
      <UCard :ui="{ body: 'divide-y divide-gray-100 dark:divide-gray-800' }">
        <template #header>
          <div class="flex items-center justify-between">
            <h3 class="text-base font-semibold leading-6 text-gray-900 dark:text-white">
              Upload Python Package (.whl or .tar.gz)
            </h3>
            <UButton
              color="neutral"
              variant="ghost"
              icon="i-lucide-x"
              class="-my-1"
              @click="isUploadOpen = false"
            />
          </div>
        </template>
        <div class="p-4 space-y-4">
          <p class="text-sm text-gray-500">
            Drag and drop your distribution packages here or upload via twine.
          </p>
          <UInput
            type="file"
            icon="i-lucide-folder"
          />
          <UTextarea
            placeholder="Release notes (optional)..."
            :rows="3"
          />
        </div>
        <template #footer>
          <div class="flex justify-end gap-3">
            <UButton
              label="Cancel"
              color="neutral"
              variant="ghost"
              @click="isUploadOpen = false"
            />
            <UButton
              label="Publish"
              color="primary"
              icon="i-lucide-rocket"
              @click="isUploadOpen = false"
            />
          </div>
        </template>
      </UCard>
    </UModal>
  </div>
</template>
