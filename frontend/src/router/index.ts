import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', redirect: '/projects' },
    { path: '/login', component: () => import('../views/auth/LoginView.vue') },
    { path: '/register', component: () => import('../views/auth/RegisterView.vue') },
    {
      path: '/projects',
      component: () => import('../components/layout/AppLayout.vue'),
      meta: { requiresAuth: true },
      children: [
        { path: '', component: () => import('../views/project/ProjectListView.vue') },
        { path: ':id', component: () => import('../views/project/ProjectDetailView.vue') },
        { path: ':id/settings', component: () => import('../views/project/ProjectSettingsView.vue') },
        { path: ':id/requirements', component: () => import('../views/workitem/RequirementListView.vue') },
        { path: ':id/bugs', component: () => import('../views/workitem/BugListView.vue') },
        { path: ':id/work-items/:wid', component: () => import('../views/workitem/WorkItemDetailView.vue') },
        { path: ':id/kanban', component: () => import('../views/kanban/KanbanView.vue') },
        { path: ':id/iterations', component: () => import('../views/iteration/IterationListView.vue') },
        { path: ':id/iterations/:iid', component: () => import('../views/iteration/IterationDetailView.vue') },
        { path: ':id/reports', component: () => import('../views/report/ReportView.vue') },
      ],
    },
    {
      path: '/notifications',
      component: () => import('../views/notification/NotificationView.vue'),
      meta: { requiresAuth: true },
    },
  ],
})

router.beforeEach((to) => {
  const auth = useAuthStore()
  if (to.meta.requiresAuth && !auth.isLoggedIn) {
    return '/login'
  }
})

export default router
