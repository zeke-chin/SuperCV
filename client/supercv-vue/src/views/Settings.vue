<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { UserConfig } from '../clipboardHelper'
import { invoke } from '@tauri-apps/api/tauri'

const config = ref<UserConfig>({
  expired_config: {
    text: 0,
    img: 0,
    file: 0,
  },
  preview_config: {
    preview_number: 10,
  },
})

const keepText = ref(false)
const keepImages = ref(false)
const keepFileList = ref(false)

const dayOptions = [
  { label: '24 小时', value: 1 },
  { label: '3 天', value: 3 },
  { label: '7 天', value: 7 },
  { label: '1 个月', value: 30 },
  { label: '3 个月', value: 90 },
]

const loading = ref(true)

const themeMode = ref('system')

const handleThemeChange = async (mode: string) => {
  themeMode.value = mode
  localStorage.setItem('theme', mode)

  document.documentElement.classList.remove('light', 'dark')

  let actualTheme: 'light' | 'dark' = 'dark'

  switch (mode) {
    case 'light':
      actualTheme = 'light'
      document.documentElement.classList.add('light')
      break
    case 'dark':
      actualTheme = 'dark'
      document.documentElement.classList.add('dark')
      break
    case 'system':
      actualTheme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
      document.documentElement.classList.add(actualTheme)
      break
  }

  await invoke('set_theme', { theme: actualTheme })
  window.dispatchEvent(new Event('theme-changed'))
}

onMounted(async () => {
  try {
    config.value = await UserConfig.getUserConfig()
    keepText.value = config.value.expired_config.text > 0
    keepImages.value = config.value.expired_config.img > 0
    keepFileList.value = config.value.expired_config.file > 0
  } catch (error) {
    console.error('加载用户配置失败:', error)
  } finally {
    loading.value = false
  }
})

onMounted(() => {
  const savedTheme = localStorage.getItem('theme') || 'system'
  themeMode.value = savedTheme
  handleThemeChange(savedTheme)
})

const saveConfig = async () => {
  try {
    await UserConfig.setUserConfig(config.value)
    console.log('设置已保存')
  } catch (error) {
    console.error('保存用户配置失败:', error)
  }
}

const handlePreviewNumberChange = async () => {
  await saveConfig()
  // 触发一个自定义事件，让其他组件知道配置已更新
  window.dispatchEvent(new Event('userConfigChanged'))
}

watch(
  [() => config.value.preview_config.preview_number, keepText, keepImages, keepFileList],
  () => {
    if (!keepText.value) config.value.expired_config.text = 0
    if (!keepImages.value) config.value.expired_config.img = 0
    if (!keepFileList.value) config.value.expired_config.file = 0
    saveConfig()
  },
  { deep: true }
)

const getDayLabel = (days: number) => {
  const option = dayOptions.find(opt => opt.value === days)
  return option ? option.label : `${days} 天`
}
</script>

<template>
  <div class="settings">
    <div class="header">
      <h2>剪贴板历史设置</h2>
    </div>

    <div v-if="loading" class="loading">加载配置中...</div>

    <div v-else class="settings-container">
      <div class="section">
        <h3>剪贴板历史：</h3>
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-label">
              <label class="switch">
                <input type="checkbox" v-model="keepText" />
                <span class="slider"></span>
              </label>
              <span>保留纯文本</span>
            </div>
            <select v-model="config.expired_config.text" :disabled="!keepText">
              <option v-for="option in dayOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </option>
            </select>
          </div>

          <div class="setting-item">
            <div class="setting-label">
              <label class="switch">
                <input type="checkbox" v-model="keepImages" />
                <span class="slider"></span>
              </label>
              <span>保留图片</span>
            </div>
            <select v-model="config.expired_config.img" :disabled="!keepImages">
              <option v-for="option in dayOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </option>
            </select>
          </div>

          <div class="setting-item">
            <div class="setting-label">
              <label class="switch">
                <input type="checkbox" v-model="keepFileList" />
                <span class="slider"></span>
              </label>
              <span>保留文件列表</span>
            </div>
            <select v-model="config.expired_config.file" :disabled="!keepFileList">
              <option v-for="option in dayOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </option>
            </select>
          </div>
        </div>
        <div class="hint-text">禁用时，剪贴板查看器仍可显示您的片段。</div>
      </div>

      <div class="section">
        <h3>预览设置：</h3>
        <div class="setting-group">
          <div class="setting-item preview-number">
            <div class="setting-label">
              <span>预览条数</span>
            </div>
            <input type="number" v-model="config.preview_config.preview_number" min="1" max="100"
              @change="handlePreviewNumberChange" />
          </div>
        </div>
      </div>

      <div class="section">
        <h3>主题设置：</h3>
        <div class="setting-group">
          <div class="theme-options">
            <div class="theme-option" :class="{ active: themeMode === 'light' }" @click="handleThemeChange('light')">
              <span class="theme-icon">☀️</span>
              <span>日间模式</span>
            </div>
            <div class="theme-option" :class="{ active: themeMode === 'dark' }" @click="handleThemeChange('dark')">
              <span class="theme-icon">🌙</span>
              <span>夜间模式</span>
            </div>
            <div class="theme-option" :class="{ active: themeMode === 'system' }" @click="handleThemeChange('system')">
              <span class="theme-icon">⚙️</span>
              <span>跟随系统</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings {
  font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  height: 100vh;
  width: 100vw;
  background: var(--bg-color);
  color: var(--text-color);
}

.header {
  text-align: center;
  padding: 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

h2 {
  margin: 0;
  font-weight: normal;
  font-size: 1.2em;
  color: #ffffff;
}

.settings-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.section {
  margin-bottom: 30px;
  width: 100%;
  max-width: 420px;
}

.section h3 {
  text-align: left;
  width: 100%;
}

h3 {
  color: #ffffff;
  font-weight: normal;
  font-size: 1em;
  margin-bottom: 15px;
}

.setting-group {
  background: rgba(45, 45, 45, 0.5);
  border-radius: 8px;
  padding: 10px;
  max-width: 580px;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px;
  border-radius: 6px;
  max-width: 550px;
}

.setting-label {
  display: flex;
  align-items: center;
  gap: 12px;
}

.setting-label span {
  color: #e0e0e0;
}

select {
  background: rgba(61, 61, 61, 0.7);
  border: none;
  border-radius: 6px;
  color: #ffffff;
  padding: 8px 12px;
  width: 120px;
  font-size: 14px;
}

input[type='number'] {
  background: rgba(61, 61, 61, 0.7);
  border: none;
  border-radius: 6px;
  color: #ffffff;
  padding: 8px 12px;
  width: 120px;
  font-size: 14px;
}

.hint-text {
  color: rgba(255, 255, 255, 0.5);
  font-size: 0.9em;
  margin-top: 10px;
  padding-left: 10px;
}

.switch {
  position: relative;
  display: inline-block;
  width: 40px;
  height: 22px;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(61, 61, 61, 0.7);
  border-radius: 22px;
  transition: .3s;
}

.slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 2px;
  bottom: 2px;
  background-color: #fff;
  border-radius: 50%;
  transition: .3s;
}

input:checked+.slider {
  background: #007AFF;
}

input:checked+.slider:before {
  transform: translateX(18px);
}

.loading {
  text-align: center;
  padding: 20px;
  color: rgba(255, 255, 255, 0.5);
}

.theme-options {
  display: flex;
  gap: 10px;
  padding: 10px;
}

.theme-option {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: rgba(61, 61, 61, 0.7);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.theme-option:hover {
  background: rgba(61, 61, 61, 0.9);
}

.theme-option.active {
  background: #007AFF;
}

.theme-icon {
  font-size: 20px;
}

.theme-option span:last-child {
  font-size: 14px;
  color: #e0e0e0;
}

.theme-option.active span {
  color: #ffffff;
}

/* 亮色主题 */
:root.light .settings {
  --bg-color: #ffffff;
  --text-color: #333333;
}

:root.light h2,
:root.light h3 {
  color: #333333;
}

:root.light .setting-label span {
  color: #333333;
}

:root.light .hint-text {
  color: #666666;
}

:root.light .theme-option span:last-child {
  color: #333333;
}

:root.light .theme-option.active span {
  color: #ffffff;
}

/* 暗色主题 */
:root.dark .settings {
  --bg-color: #2c2c2c;
  --text-color: #ffffff;
}

/* 使用变量 */
body {
  background-color: var(--bg-color);
  color: var(--text-color);
}

/* 修改亮色主题下的样式 */
:root.light .setting-group {
  background: rgba(200, 200, 200, 0.5);
}

:root.light select,
:root.light input[type='number'] {
  background: rgba(180, 180, 180, 0.7);
  color: #000000;
}

:root.light .theme-option {
  background: rgba(180, 180, 180, 0.7);
}

:root.light .theme-option span {
  color: #000000;
}

:root.light .theme-option.active {
  background: #007AFF;
}

:root.light .theme-option.active span {
  color: #ffffff;
}
</style>
