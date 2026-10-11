<template>
  <UDashboardPanel id="customers">
    <template #header>
      <UDashboardNavbar title="Customers">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>
    <template #body>
      <UEmpty
        icon="i-lucide-users"
        title="No customers yet"
        description="Customers will appear here once they are added."
      />
    </template>
  </UDashboardPanel>
</template>
