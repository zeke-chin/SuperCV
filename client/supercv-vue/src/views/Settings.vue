<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { UserConfig } from '../clipboardHelper'

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

const saveConfig = async () => {
  try {
    await UserConfig.setUserConfig(config.value)
    console.log('设置已保存')
  } catch (error) {
    console.error('保存用户配置失败:', error)
  }
}

watch(
  [config, keepText, keepImages, keepFileList],
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
    <h2>剪贴板历史设置</h2>

    <div v-if="loading" class="loading">加载配置中...</div>

    <div v-else>
      <div class="setting-group">
        <div class="setting-item">
          <label class="switch">
            <input type="checkbox" v-model="keepText" />
            <span class="slider"></span>
          </label>
          <span>保留纯文本</span>
          <select v-model="config.expired_config.text" :disabled="!keepText">
            <option
              v-for="option in dayOptions"
              :key="option.value"
              :value="option.value"
            >
              {{ option.label }}
            </option>
          </select>
          <span class="config-value"
            >当前值: {{ getDayLabel(config.expired_config.text) }}</span
          >
        </div>

        <div class="setting-item">
          <label class="switch">
            <input type="checkbox" v-model="keepImages" />
            <span class="slider"></span>
          </label>
          <span>保留图片</span>
          <select v-model="config.expired_config.img" :disabled="!keepImages">
            <option
              v-for="option in dayOptions"
              :key="option.value"
              :value="option.value"
            >
              {{ option.label }}
            </option>
          </select>
          <span class="config-value"
            >当前值: {{ getDayLabel(config.expired_config.img) }}</span
          >
        </div>

        <div class="setting-item">
          <label class="switch">
            <input type="checkbox" v-model="keepFileList" />
            <span class="slider"></span>
          </label>
          <span>保留文件列表</span>
          <select
            v-model="config.expired_config.file"
            :disabled="!keepFileList"
          >
            <option
              v-for="option in dayOptions"
              :key="option.value"
              :value="option.value"
            >
              {{ option.label }}
            </option>
          </select>
          <span class="config-value"
            >当前值: {{ getDayLabel(config.expired_config.file) }}</span
          >
        </div>
      </div>

      <div class="setting-item preview-number">
        <label>预览条数：</label>
        <input
          type="number"
          v-model="config.preview_config.preview_number"
          min="1"
          max="100"
        />
        <span class="config-value"
          >当前值: {{ config.preview_config.preview_number }}</span
        >
      </div>
    </div>
  </div>
</template>
<style scoped>
.settings {
  font-family: Arial, sans-serif;
  max-width: 500px;
  margin: 0 auto;
  padding: 20px;
  background-color: #f5f5f5;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

h2 {
  color: #333;
  margin-bottom: 20px;
}

.setting-group {
  background-color: white;
  border-radius: 8px;
  padding: 15px;
  margin-bottom: 20px;
}

.setting-item {
  display: flex;
  align-items: center;
  margin-bottom: 15px;
}

.setting-item:last-child {
  margin-bottom: 0;
}

.setting-item span {
  margin-left: 10px;
  margin-right: auto;
}

select,
input[type='number'] {
  padding: 5px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.preview-number {
  background-color: white;
  padding: 15px;
  border-radius: 8px;
}

.preview-number label {
  margin-right: 10px;
}

.switch {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: 0.4s;
  border-radius: 34px;
}

.slider:before {
  position: absolute;
  content: '';
  height: 16px;
  width: 16px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  transition: 0.4s;
  border-radius: 50%;
}

input:checked + .slider {
  background-color: #2196f3;
}

input:checked + .slider:before {
  transform: translateX(26px);
}
</style>
