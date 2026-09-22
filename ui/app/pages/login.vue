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
    name: 'email',
    type: 'email',
    label: 'Email',
    placeholder: 'Enter your email',
    required: true
  },
  {
    name: 'password',
    label: 'Password',
    type: 'password',
    placeholder: 'Enter your password',
    required: true
  },
  {
    name: 'remember',
    label: 'Remember me',
    type: 'checkbox'
  }
]

// TODO
// const providers = [{
//   label: 'Google',
//   icon: 'i-simple-icons-google',
//   onClick: () => {
//     toast.add({ title: 'Google', description: 'Login with Google' })
//   }
// }, {
//   label: 'GitHub',
//   icon: 'i-simple-icons-github',
//   onClick: () => {
//     toast.add({ title: 'GitHub', description: 'Login with GitHub' })
//   }
// }]

const schema = z.object({
  email: z.email('Invalid email'),
  password: z.string('Password is required').min(8, 'Must be at least 8 characters')
})

type Schema = z.output<typeof schema>

async function onSubmit(payload: FormSubmitEvent<Schema>) {
  errorMessage.value = ''
  isSubmitting.value = true

  try {
    const response = await $fetch<{ access_token: string }>('/api/v1/auth/login', {
      baseURL: config.public.apiBase,
      method: 'POST',
      body: payload.data
    })

    accessToken.value = response.access_token
    await navigateTo('/')
  } catch (error: unknown) {
    const status
      = typeof error === 'object' && error !== null && 'response' in error
        ? (error as { response?: { status?: number } }).response?.status
        : undefined

    errorMessage.value
      = status === 401
        ? 'The email or password is incorrect.'
        : 'Unable to sign in right now. Please try again.'
  } finally {
    isSubmitting.value = false
  }
}
</script>

<template>
  <div class="flex flex-col items-center justify-center gap-4 p-4 min-h-screen">
    <UPageCard class="w-full max-w-md">
      <UAuthForm
        :schema="schema"
        title="Welcome back!"
        description="Enter your credentials to access your account."
        icon="i-lucide-lock"
        :fields="fields"
        :loading="isSubmitting"
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
        <!-- <template #password-hint>
          <ULink to="#" class="text-primary font-medium" tabindex="-1">Forgot password?</ULink>
        </template> -->
        <!-- <template #footer>
          By signing in, you agree to our <ULink to="#" class="text-primary font-medium">Terms of Service</ULink>.
        </template> -->
      </UAuthForm>
    </UPageCard>
  </div>
</template>
