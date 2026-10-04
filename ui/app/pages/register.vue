<script setup lang="ts">
import * as z from 'zod'
import type { FormSubmitEvent, AuthFormField } from '@nuxt/ui'

const config = useRuntimeConfig()
const accessToken = useCookie<string | null>('nyxos_access_token', {
  maxAge: 60 * 60,
  sameSite: 'lax'
})
const isSubmitting = ref(false)
const errorMessage = ref('')

const fields: AuthFormField[] = [
  {
    name: 'name',
    type: 'text',
    label: 'Full name',
    placeholder: 'Ada Lovelace',
    required: true
  },
  {
    name: 'email',
    type: 'email',
    label: 'Email address',
    placeholder: 'you@example.com',
    required: true
  },
  {
    name: 'password',
    type: 'password',
    label: 'Password',
    placeholder: 'Create a secure password',
    required: true
  },
  {
    name: 'confirmPassword',
    type: 'password',
    label: 'Confirm password',
    placeholder: 'Enter your password again',
    required: true
  }
]

const schema = z
  .object({
    name: z.string('Full name is required').trim().min(1, 'Full name is required'),
    email: z.email('Enter a valid email address'),
    password: z.string('Password is required').min(8, 'Use at least 8 characters'),
    confirmPassword: z.string('Please confirm your password')
  })
  .refine(data => data.password === data.confirmPassword, {
    path: ['confirmPassword'],
    message: 'Passwords do not match'
  })

type Schema = z.output<typeof schema>

async function onSubmit(payload: FormSubmitEvent<Schema>) {
  errorMessage.value = ''
  isSubmitting.value = true

  try {
    const { name, email, password } = payload.data

    await $fetch('/api/v1/users', {
      baseURL: config.public.apiBase,
      method: 'POST',
      body: { name, email, password }
    })

    const response = await $fetch<{ access_token: string }>('/api/v1/auth/login', {
      baseURL: config.public.apiBase,
      method: 'POST',
      body: { email, password }
    })

    accessToken.value = response.access_token
    await navigateTo('/dashboard')
  } catch (error: unknown) {
    const status
      = typeof error === 'object' && error !== null && 'response' in error
        ? (error as { response?: { status?: number } }).response?.status
        : undefined

    errorMessage.value
      = status === 409
        ? 'An account with this email already exists.'
        : 'Unable to create your account right now. Please try again.'
  } finally {
    isSubmitting.value = false
  }
}
</script>

<template>
  <div class="relative flex min-h-[calc(100vh-9rem)] items-center justify-center overflow-hidden px-4 py-12 sm:px-6">
    <div
      class="pointer-events-none absolute inset-0 -z-10 overflow-hidden"
      aria-hidden="true"
    >
      <div class="absolute left-1/2 top-0 h-80 w-80 -translate-x-1/2 rounded-full bg-primary/10 blur-3xl" />
      <div class="absolute -bottom-32 -right-20 h-72 w-72 rounded-full bg-secondary/10 blur-3xl" />
    </div>

    <UPageCard class="w-full max-w-lg shadow-xl shadow-primary/5">
      <UAuthForm
        :schema="schema"
        title="Create your account"
        description="Join nyxos to securely manage and deploy your Python packages."
        icon="i-lucide-user-plus"
        :fields="fields"
        :loading="isSubmitting"
        submit-label="Create account"
        @submit="onSubmit"
      >
        <template #validation>
          <UAlert
            v-if="errorMessage"
            color="error"
            variant="subtle"
            :description="errorMessage"
            icon="i-lucide-circle-alert"
          />
        </template>

        <template #footer>
          <p class="text-center text-sm text-muted">
            Already have an account?
            <ULink
              to="/login"
              class="font-medium text-primary hover:underline"
            >
              Sign in
            </ULink>
          </p>
        </template>
      </UAuthForm>
    </UPageCard>
  </div>
</template>
