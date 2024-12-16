<script setup lang="ts">
import { ref, onMounted, computed, watch, Ref, onUnmounted, nextTick } from 'vue'
import { appWindow, Theme } from '@tauri-apps/api/window'
import { ClipboardHelper, ClipboardEntry, UserConfig } from '../clipboardHelper'
import { invoke } from '@tauri-apps/api/tauri'
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'

const textInput = ref('')
const clipboardEntries = ref<ClipboardEntry[]>([])
const selectedIndex = ref(-1)
let isKeyboardSelection = ref(true)
const previewNumber = ref(10)
const tempDisplayCount = ref(0)

function openSettings() {
  invoke('rs_invoke_open_settings')
}

const displayContent = computed(() => {
  if (selectedIndex.value >= 0 && selectedIndex.value < clipboardEntries.value.length) {
    const item = clipboardEntries.value[selectedIndex.value]
    if (item.type == 2) {
      return item.path.replaceAll(', ', '\n')
    }
    return item.content
  }
  return ''
})

const selectedEntry = computed(() => {
  if (selectedIndex.value >= 0 && selectedIndex.value < clipboardEntries.value.length) {
    return clipboardEntries.value[selectedIndex.value]
  }
  return null
})

const isImageEntry = computed(() => {
  console.log('imageSrc', selectedEntry.value)
  return selectedEntry.value?.type === 1
})

const imageSrc = computed(() => {
  if (isImageEntry.value && selectedEntry.value) {
    console.log('imageSrc', selectedEntry.value.path)
    return convertFileSrc(selectedEntry.value.path)
  }
  return ''
})

async function getClipboardContent() {
  try {
    const displayNum = tempDisplayCount.value || previewNumber.value
    clipboardEntries.value = await ClipboardHelper.getClipboardEntries(displayNum)
    selectedIndex.value = -1
  } catch (error) {
    console.error('Failed to get clipboard content:', error)
    clipboardEntries.value = []
  }
}

async function searchClipboard() {
  try {
    const displayNum = tempDisplayCount.value || previewNumber.value
    clipboardEntries.value = await ClipboardHelper.searchClipboardEntries(
      textInput.value,
      displayNum
    )
    selectedIndex.value = -1
  } catch (error) {
    console.error('Failed to search clipboard content:', error)
    clipboardEntries.value = []
  }
}

async function copyToClipboardAndHide(item: ClipboardEntry) {
  try {
    if (item.type == 0) {
      await navigator.clipboard.writeText(item.content)
      console.log('使用navigator set clipboard')
    } else {
      await ClipboardHelper.setClipboardEntriy(item)
      console.log('使用rust set clipboard')
    }
    await appWindow.hide()
  } catch (err) {
    console.error('Failed to copy text or hide window: ', err)
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'ArrowUp' || e.key === 'ArrowDown') {
    e.preventDefault()
    isKeyboardSelection.value = true
    if (e.key === 'ArrowUp' && selectedIndex.value > 0) {
      selectedIndex.value--
    } else if (e.key === 'ArrowDown' && selectedIndex.value < clipboardEntries.value.length) {
      selectedIndex.value++
    }
  } else if (e.key === 'Enter' || ((e.metaKey || e.ctrlKey) && e.key === 'c')) {
    e.preventDefault()
    if (selectedIndex.value !== -1) {
      if (selectedIndex.value === clipboardEntries.value.length) {
        handleLoadMore()
        return
      }
      const selectedItem = clipboardEntries.value[selectedIndex.value]
      copyToClipboardAndHide(selectedItem)
    }
  } else if (e.key === 'Escape') {
    appWindow.hide()
  } else if ((e.metaKey || e.ctrlKey) && e.key === ',') {
    e.preventDefault()
    try {
      openSettings()
    } catch (e) {
      console.error('Failed to open settings:', e)
    }
  }
}

function handleMouseMove() {
  isKeyboardSelection.value = false
}

const inputRef = ref<HTMLInputElement | null>(null)
const theme: Ref<Theme> = ref('light')

const updateTheme = async () => {
  const savedTheme = localStorage.getItem('theme') || 'system'
  const themeValue = savedTheme === 'system'
    ? (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light')
    : savedTheme as 'light' | 'dark'

  // console.log('Current theme:', themeValue)
  theme.value = themeValue
}

async function updatePreviewNumber() {
  try {
    const config = await UserConfig.getUserConfig()
    previewNumber.value = config.preview_config.preview_number
    console.log('updatePreviewNumber', previewNumber.value)
    if (textInput.value.trim() !== '') {
      await searchClipboard()
    } else {
      await getClipboardContent()
    }
  } catch (error) {
    console.error('Failed to get user config:', error)
  }
}

function handleLoadMore() {
  const currentIndex = selectedIndex.value  // 保存当前选中的位置
  tempDisplayCount.value = (tempDisplayCount.value || previewNumber.value) + 5
  if (textInput.value.trim() !== '') {
    searchClipboard().then(() => {
      selectedIndex.value = currentIndex  // 恢复选中位置
    })
  } else {
    getClipboardContent().then(() => {
      selectedIndex.value = currentIndex  // 恢复选中位置
    })
  }
}

// 添加一个函数来处理滚动
function scrollToSelected() {
  // 等待 DOM 更新
  nextTick(() => {
    const selectedElement = document.querySelector('.paste-content-item-selected')
    if (selectedElement) {
      selectedElement.scrollIntoView({
        block: 'nearest',  // 使用 'nearest' 实现更平滑的滚动
        behavior: 'smooth'
      })
    }
  })
}

// 在 selectedIndex 改变时触发滚动
watch(selectedIndex, () => {
  scrollToSelected()
})

onMounted(async () => {
  await updatePreviewNumber()
  const unlisten = await listen('userConfigChanged', async () => {
    console.log('接收到 userConfigChanged 事件')
    await updatePreviewNumber()
  })
  document.addEventListener('keydown', handleKeydown)
  document.addEventListener('mousemove', handleMouseMove)

  window.addEventListener('theme-changed', async () => {
    await updateTheme()
  })

  await appWindow.onFocusChanged(async ({ payload: focused }) => {
    if (focused) {
      textInput.value = ''
      tempDisplayCount.value = 0
      await getClipboardContent()
      inputRef.value?.focus()
    }
    updateTheme()
  })

  inputRef.value?.focus()

  updateTheme()

  onUnmounted(() => {
    unlisten()
  })
})

watch(textInput, () => {
  if (textInput.value.trim() !== '') {
    searchClipboard()
  } else {
    getClipboardContent()
  }
})

const selectedTimestamp = computed(() => {
  if (selectedIndex.value >= 0 && selectedIndex.value < clipboardEntries.value.length) {
    return clipboardEntries.value[selectedIndex.value].timestamp
  }
  return null
})

const formattedTimestamp = computed(() => {
  if (selectedTimestamp.value) {
    // 将秒转换为毫秒
    const milliseconds = selectedTimestamp.value * 1000
    const date = new Date(milliseconds)

    // 使用更易读的格式
    const options: Intl.DateTimeFormatOptions = {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false,
    }

    return date.toLocaleString(undefined, options)
  }
  return ''
})

const pasteItemIcon = computed(() => (type: number) => {
  switch (type) {
    case 0:
      return '📝'
    case 1:
      return '🖼️'
    case 2:
      return '📁'
    case -1:
      return '🔄'
    default:
      return '📝'
  }
})

const handleSelectPasteItem = (index: number, item: any) => {
  selectedIndex.value = index
  if (index === clipboardEntries.value.length) {
    // 如果是 "加载更多" 选项
    handleLoadMore()
  } else {
    // 如果是普通剪贴板项
    copyToClipboardAndHide(item)
  }
}

const hoverSettings = ref(false)
</script>

<template>
  <div class="main" :class="{
    'main-dark': theme === 'dark',
    'main-light': theme === 'light',
  }" data-tauri-drag-region>
    <div class="paste-filter">
      <input class="paste-filter-input" ref="inputRef" v-model="textInput" />
    </div>
    <div class="paste-content">
      <div class="paste-content-list">
        <div class="paste-content-item" :class="{
          'paste-content-item-selected': selectedIndex === index,
        }" v-for="(item, index) in clipboardEntries" :key="item.id" @mouseover="() => {
          selectedIndex = index
        }
          " @click="handleSelectPasteItem(index, item)">
          <div class="paste-item-icon">
            {{ pasteItemIcon(item.type) }}
          </div>
          <div class="paste-item-text">
            {{ item.content }}
          </div>
        </div>
        <div v-if="clipboardEntries.length > 0 && !textInput.trim()" class="paste-content-item" :class="{
          'paste-content-item-selected': selectedIndex === clipboardEntries.length,
        }" @mouseover="() => { selectedIndex = clipboardEntries.length }" @click="handleLoadMore">
          <div class="paste-item-icon">
            {{ pasteItemIcon(-1) }}
          </div>
          <div class="paste-item-text">
            加载更多...
          </div>
        </div>
      </div>
      <div class="paste-content-desc">
        <div class="desc-wrapper">
          <img v-if="isImageEntry" :src="imageSrc" alt="Clipboard image" />
          <pre v-else>{{ displayContent }}</pre>
        </div>
        <div class="timestamp-wrapper" data-tauri-drag-region>
          <p class="timestamp-content" :class="{
            'timestamp-content-light': theme === 'light',
            'timestamp-content-dark': theme === 'dark',
          }">
            <span v-if="selectedTimestamp">
              {{ formattedTimestamp }}
            </span>
            <span v-else> 输入值筛选剪贴板内容 </span>
          </p>
        </div>
      </div>
    </div>
    <div class="paste-settings" @mouseenter="() => {
      hoverSettings = true
    }
      " @mouseleave="() => {
        hoverSettings = false
      }
        ">
      <img v-if="theme === 'dark' || hoverSettings" class="paste-settings-icon paste-settings-icon-normal"
        src="../assets/settings-hover.svg" alt="Settings" @click="openSettings" />
      <img v-else class="paste-settings-icon paste-settings-icon-hover" src="../assets/settings.svg" alt="Settings"
        @click="openSettings" />
    </div>
  </div>
</template>

<style>
/* 主窗口尺寸设置 - 默认宽度100%, 高度100vh */
.main {
  width: 100%;
  height: 100vh;
  padding: 15px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  position: relative;
  border-radius: 8px;
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
}

.main-light {
  color: #000;
  background-color: rgba(255, 255, 255, 0.7);
}

.main-dark {
  color: #fff;
  background-color: rgba(44, 44, 44, 0.4);
}

.paste-settings {
  position: absolute;
  right: 10px;
  bottom: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  border-radius: 5px;
  padding: 3px;
}

.paste-settings:hover {
  background-color: rgba(88, 206, 141, 0.7);
}

.paste-settings-icon {
  width: 25px;
}

.paste-filter {
  width: 100%;
}

.paste-filter-input {
  width: 100%;
  height: 30px;
  border-radius: 5px;
  border: none;
  box-shadow: none;
  outline: none;
  background: rgba(0, 0, 0, 0.2);
  padding-left: 5px;
  font-size: 18px;
}

.paste-content {
  flex: 1;
  display: flex;
  margin-top: 10px;
  column-gap: 10px;
  height: 0;
}

.paste-content-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  row-gap: 5px;
  overflow-y: auto;
}

.paste-content-desc {
  flex: 1;
  display: flex;
  flex-direction: column;
  width: 0;
  row-gap: 5px;
}

.desc-wrapper {
  flex: 1;
  height: 0;
  flex-shrink: 0;
  overflow-y: auto;
}

.desc-wrapper pre {
  font-size: 13.5px;
  font-family: system-ui, sans-serif;
  white-space: pre-wrap;
  word-wrap: break-word;
  margin: 0;
  line-height: 1.4;
}

.desc-wrapper img {
  display: block;
  /* 设置 img 为块级元素 */
  margin: auto;
  /* 自动外距实现水平居中 */
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  /* 垂直居中（如果父元素是 flex 或 grid 容器） */
  align-self: center;
}

.timestamp-wrapper {
  /* height: 20px; */
  display: flex;
  align-items: center;
  justify-content: center;
}

.timestamp-content-light {
  color: #000000;
}

.timestamp-content-dark {
  color: #fff;
}

.paste-content-item {
  width: 100%;
  height: 25px;
  display: flex;
  align-items: center;
  border-radius: 5px;
  padding-left: 5px;
  font-weight: 300;
  font-size: 16px;
  cursor: default;
  column-gap: 5px;
  flex-shrink: 0;
}

.paste-content-item:hover {
  background: rgba(88, 206, 141, 0.7);
  color: #fff;
}

.paste-content-item-selected {
  background: rgba(88, 206, 141, 0.7);
  color: #fff;
}

.paste-item-icon {
  width: 20px;
  font-size: 15px;
}

.paste-item-text {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  width: 0;
}

.paste-item-shortcut {
  width: 30px;
}

/* 修改输入框样式 */
.main-light .paste-filter-input {
  background: rgba(255, 255, 255, 0.3);
  color: #000;
}

.main-dark .paste-filter-input {
  background: rgba(0, 0, 0, 0.2);
  color: #fff;
}

.load-more {
  text-align: center;
  cursor: pointer;
}

.load-more .paste-item-text {
  justify-content: center;
}
</style>
