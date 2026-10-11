<script setup lang="ts">
import type { TableColumn } from '@nuxt/ui'

const config = useRuntimeConfig()
const users = ref<User[]>([])
const isLoading = ref(true)
const isSaving = ref(false)
const errorMessage = ref('')
const isModalOpen = ref(false)
const form = ref<UserForm>({ name: '', email: '', password: '' })
const editingUser = ref<User | null>(null)

interface User {
  id: number
  name: string
  email: string
}

interface UserForm {
  name: string
  email: string
  password: string
}

const columns: TableColumn<User>[] = [
  { accessorKey: 'name', header: 'Name' },
  { accessorKey: 'email', header: 'Email address' },
  { id: 'actions', header: '' }
]

async function loadUsers() {
  isLoading.value = true
  errorMessage.value = ''

  try {
    const response = await $fetch<User[]>('/api/v1/users', {
      baseURL: config.public.apiBase
    })
    users.value = response
  } catch (error: unknown) {
    errorMessage.value = getErrorMessage(error, 'Unable to load users. Please try again.')
  } finally {
    isLoading.value = false
  }
}

async function saveUser() {
  errorMessage.value = ''

  if (!form.value.name.trim() || !form.value.email.trim()) {
    errorMessage.value = 'Name and email address are required.'
    return
  }

  if (!editingUser.value && !form.value.password) {
    errorMessage.value = 'A password is required for new users.'
    return
  }

  isSaving.value = true

  try {
    if (editingUser.value) {
      const updatedUser = await $fetch<User>(`/api/v1/users/${editingUser.value.id}`, {
        baseURL: config.public.apiBase,
        method: 'PATCH',
        body: {
          name: form.value.name.trim(),
          email: form.value.email.trim()
        }
      })
      users.value = users.value.map(user => user.id === updatedUser.id ? updatedUser : user)
    } else {
      const createdUser = await $fetch<User>('/api/v1/users', {
        baseURL: config.public.apiBase,
        method: 'POST',
        body: {
          name: form.value.name.trim(),
          email: form.value.email.trim(),
          password: form.value.password
        }
      })
      users.value = [...users.value, createdUser]
    }

    isModalOpen.value = false
  } catch (error: unknown) {
    errorMessage.value = getErrorMessage(error, 'Unable to save this user. Please try again.')
  } finally {
    isSaving.value = false
  }
}

function getErrorMessage(error: unknown, fallback: string) {
  if (typeof error === 'object' && error !== null && 'response' in error) {
    const response = error.response
    if (typeof response === 'object' && response !== null && 'status' in response) {
      return response.status === 409
        ? 'A user with this email already exists.'
        : fallback
    }
  }

  return fallback
}

function openCreateModal() {
  editingUser.value = null
  form.value = { name: '', email: '', password: '' }
  errorMessage.value = ''
  isModalOpen.value = true
}

function closeModal() {
  if (!isSaving.value) {
    isModalOpen.value = false
  }
}

await loadUsers()
</script>

<template>
  <div class="space-y-6">
    <div class="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
      <div>
        <h2 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
          Users
        </h2>
        <p class="text-gray-500 dark:text-gray-400">
          Create, update, and remove accounts that can access your private repository.
        </p>
      </div>
      <UButton
        label="Add user"
        icon="i-lucide-user-plus"
        @click="openCreateModal"
      />
    </div>

    <UAlert
      v-if="errorMessage"
      color="error"
      variant="subtle"
      :description="errorMessage"
      icon="i-lucide-circle-alert"
    />

    <UCard>
      <UTable
        :data="users"
        :columns="columns"
        :loading="isLoading"
        class="w-full"
      >
        <template #name-data="{ row }">
          <div class="flex items-center gap-3">
            <UAvatar
              :text="row.original.name.slice(0, 2).toUpperCase()"
              size="sm"
            />
            <span class="font-medium text-gray-900 dark:text-white">
              {{ row.original.name }}
            </span>
          </div>
        </template>

        <template #email-data="{ row }">
          <span class="text-muted">{{ row.original.email }}</span>
        </template>

        <!-- <template #actions-data="{ row }">
          <div class="flex justify-end gap-1">
            <UButton
              color="neutral"
              variant="ghost"
              icon="i-lucide-pencil"
              aria-label="Edit user"
              @click="openEditModal(row.original)"
            />
            <UButton
              color="error"
              variant="ghost"
              icon="i-lucide-trash-2"
              aria-label="Delete user"
              @click="openDeleteModal(row.original)"
            />
          </div>
        </template> -->

        <template #empty>
          <div class="py-10 text-center">
            <UIcon
              name="i-lucide-users"
              class="mx-auto size-8 text-dimmed"
            />
            <p class="mt-2 font-medium text-highlighted">
              No users yet
            </p>
            <p class="text-sm text-muted">
              Add a user to grant access to your repository.
            </p>
          </div>
        </template>
      </UTable>
    </UCard>

    <UModal v-model:open="isModalOpen">
      <template #content>
        <UCard>
          <template #header>
            <div class="flex items-center justify-between">
              <div>
                <h3 class="font-semibold text-highlighted">
                  {{ editingUser ? 'Edit user' : 'Add user' }}
                </h3>
                <p class="text-sm text-muted">
                  {{ editingUser ? 'Update this user’s name or email address.' : 'Create an account for a repository user.' }}
                </p>
              </div>
              <UButton
                color="neutral"
                variant="ghost"
                icon="i-lucide-x"
                aria-label="Close"
                @click="closeModal"
              />
            </div>
          </template>

          <form
            class="space-y-4"
            @submit.prevent="saveUser"
          >
            <UFormField
              label="Name"
              required
            >
              <UInput
                v-model="form.name"
                class="w-full"
                placeholder="Ada Lovelace"
                autocomplete="name"
              />
            </UFormField>
            <UFormField
              label="Email address"
              required
            >
              <UInput
                v-model="form.email"
                class="w-full"
                type="email"
                placeholder="ada@example.com"
                autocomplete="email"
              />
            </UFormField>
            <UFormField
              v-if="!editingUser"
              label="Password"
              required
            >
              <UInput
                v-model="form.password"
                class="w-full"
                type="password"
                placeholder="At least 8 characters"
                autocomplete="new-password"
              />
            </UFormField>
            <div class="flex justify-end gap-3 pt-2">
              <UButton
                label="Cancel"
                color="neutral"
                variant="ghost"
                type="button"
                @click="closeModal"
              />
              <UButton
                :label="editingUser ? 'Save changes' : 'Create user'"
                :loading="isSaving"
                type="submit"
              />
            </div>
          </form>
        </UCard>
      </template>
    </UModal>

    <!-- <UModal v-model:open="isDeleteModalOpen">
      <template #content>
        <UCard>
          <template #header>
            <h3 class="font-semibold text-highlighted">
              Delete user?
            </h3>
          </template>
          <p class="text-sm text-muted">
            This permanently removes
            <strong class="text-highlighted">{{ userToDelete?.name }}</strong>
            and their access to the repository.
          </p>
          <div class="mt-6 flex justify-end gap-3">
            <UButton
              label="Cancel"
              color="neutral"
              variant="ghost"
              @click="isDeleteModalOpen = false"
            />
            <UButton
              label="Delete user"
              color="error"
              icon="i-lucide-trash-2"
              :loading="isDeleting"
              @click="deleteUser"
            />
          </div>
        </UCard>
      </template>
    </UModal> -->
  </div>
</template>
