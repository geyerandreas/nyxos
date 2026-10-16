<script setup lang="ts">
useHead({
  title: 'Nyxos — private Python packages'
})
</script>

<template>
  <div class="min-h-screen overflow-hidden bg-default">
    <AppHeader />

    <main>
      <section class="relative isolate">
        <div
          class="pointer-events-none absolute inset-x-0 top-0 -z-10 h-[38rem] overflow-hidden"
          aria-hidden="true"
        >
          <div class="absolute left-1/2 top-[-18rem] size-[42rem] -translate-x-1/2 rounded-full bg-primary/10 blur-3xl" />
          <div class="absolute right-[-12rem] top-32 size-80 rounded-full bg-secondary/10 blur-3xl" />
        </div>

        <div class="mx-auto grid w-full max-w-6xl gap-16 px-4 pb-24 pt-20 sm:px-6 lg:grid-cols-[1.05fr_0.95fr] lg:items-center lg:px-8 lg:pb-32 lg:pt-28">
          <div>
            <UBadge
              color="primary"
              variant="subtle"
              icon="i-lucide-sparkles"
              label="The simple private package registry"
            />
            <h1 class="mt-6 max-w-2xl text-5xl font-semibold tracking-tight text-highlighted sm:text-6xl">
              Ship Python packages with confidence.
            </h1>
            <p class="mt-6 max-w-xl text-lg leading-8 text-muted">
              Nyxos gives your team a secure, focused home for the packages you build and the software you ship.
            </p>
            <div class="mt-8 flex flex-wrap items-center gap-3">
              <UButton
                to="/register"
                size="lg"
                label="Create your account"
                trailing-icon="i-lucide-arrow-right"
              />
              <UButton
                to="/login"
                size="lg"
                color="neutral"
                variant="ghost"
                label="Sign in"
              />
            </div>
            <p class="mt-5 text-sm text-dimmed">
              Built for teams that value clarity, control, and great tooling.
            </p>
          </div>

          <div class="relative">
            <div class="rounded-2xl border border-default bg-elevated/70 p-2 shadow-2xl shadow-primary/10 backdrop-blur">
              <div class="rounded-xl bg-[#18181b] p-6 text-sm text-zinc-300">
                <div class="mb-8 flex items-center gap-2">
                  <span class="size-3 rounded-full bg-red-400/80" />
                  <span class="size-3 rounded-full bg-amber-400/80" />
                  <span class="size-3 rounded-full bg-emerald-400/80" />
                  <span class="ml-auto font-mono text-xs text-zinc-500">terminal</span>
                </div>
                <p class="font-mono text-zinc-500">
                  $ pip install nyxos-client
                </p>
                <p class="mt-4 font-mono">
                  <span class="text-emerald-400">✓</span>
                  Installing your team's packages
                </p>
                <p class="mt-2 font-mono text-zinc-500">
                  Successfully installed acme-data-2.4.0
                </p>
                <div class="mt-8 border-t border-zinc-700/70 pt-5">
                  <p class="font-mono text-xs text-zinc-500">
                    Your private index
                  </p>
                  <p class="mt-2 font-mono text-primary-300">
                    packages.nyxos.dev/simple/
                  </p>
                </div>
              </div>
            </div>
            <div class="absolute -bottom-5 -left-5 hidden items-center gap-3 rounded-xl border border-default bg-default px-4 py-3 shadow-lg sm:flex">
              <span class="flex size-9 items-center justify-center rounded-lg bg-success/10 text-success">
                <UIcon
                  name="i-lucide-shield-check"
                  class="size-5"
                />
              </span>
              <div>
                <p class="text-sm font-medium text-highlighted">
                  Private by default
                </p>
                <p class="text-xs text-muted">
                  Only your team can access it
                </p>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section class="border-y border-default bg-elevated/30">
        <div class="mx-auto grid w-full max-w-6xl gap-8 px-4 py-12 sm:px-6 md:grid-cols-3 lg:px-8">
          <div class="flex gap-4">
            <UIcon
              name="i-lucide-lock-keyhole"
              class="mt-1 size-5 shrink-0 text-primary"
            />
            <div>
              <h2 class="font-semibold text-highlighted">
                Private and secure
              </h2>
              <p class="mt-1 text-sm leading-6 text-muted">
                Keep internal code away from public registries.
              </p>
            </div>
          </div>
          <div class="flex gap-4">
            <UIcon
              name="i-lucide-zap"
              class="mt-1 size-5 shrink-0 text-primary"
            />
            <div>
              <h2 class="font-semibold text-highlighted">
                Ready in minutes
              </h2>
              <p class="mt-1 text-sm leading-6 text-muted">
                A clean workflow without unnecessary ceremony.
              </p>
            </div>
          </div>
          <div class="flex gap-4">
            <UIcon
              name="i-lucide-users"
              class="mt-1 size-5 shrink-0 text-primary"
            />
            <div>
              <h2 class="font-semibold text-highlighted">
                Made for teams
              </h2>
              <p class="mt-1 text-sm leading-6 text-muted">
                Give the right people access to the right packages.
              </p>
            </div>
          </div>
        </div>
      </section>
    </main>
  </div>
</template>
