import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Dashboard',
      component: () => import('@/views/Dashboard.vue')
    },
    {
      path: '/powershell',
      name: 'PowerShell',
      component: () => import('@/views/PowerShell.vue')
    },
    {
      path: '/registry',
      name: 'Registry',
      component: () => import('@/views/Registry.vue')
    },
    {
      path: '/services',
      name: 'Services',
      component: () => import('@/views/Services.vue')
    },
    {
      path: '/processes',
      name: 'Processes',
      component: () => import('@/views/Processes.vue')
    },
    {
      path: '/network',
      name: 'Network',
      component: () => import('@/views/Network.vue')
    },
    {
      path: '/disk',
      name: 'Disk',
      component: () => import('@/views/Disk.vue')
    },
    {
      path: '/quick-actions',
      name: 'QuickActions',
      component: () => import('@/views/QuickActions.vue')
    },
    {
      path: '/optimization',
      name: 'Optimization',
      component: () => import('@/views/Optimization.vue')
    },
    {
      path: '/advanced',
      name: 'Advanced',
      component: () => import('@/views/Advanced.vue')
    },
    {
      path: '/settings',
      name: 'Settings',
      component: () => import('@/views/Settings.vue')
    }
  ]
})

export default router
