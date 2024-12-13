<template>
  <router-view></router-view>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'

onMounted(() => {
  // 从本地存储获取主题设置
  const savedTheme = localStorage.getItem('theme') || 'system'

  // 移除可能存在的主题类
  document.documentElement.classList.remove('light', 'dark')

  // 应用主题
  if (savedTheme === 'system') {
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.add('light')
    }
  } else {
    document.documentElement.classList.add(savedTheme)
  }

  // 监听系统主题变化
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    if (localStorage.getItem('theme') === 'system') {
      document.documentElement.classList.remove('light', 'dark')
      document.documentElement.classList.add(e.matches ? 'dark' : 'light')
    }
  })
})
</script>