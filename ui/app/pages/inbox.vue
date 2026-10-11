<template>
  <UDashboardPanel id="inbox">
    <template #header>
      <UDashboardNavbar title="Inbox">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>
    <template #body>
      <UEmpty
        icon="i-lucide-inbox"
        title="Your inbox is empty"
        description="New messages will appear here."
      />
    </template>
  </UDashboardPanel>
</template>
