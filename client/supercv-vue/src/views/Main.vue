<script setup lang="ts">
import { ref, onMounted, computed, watch, Ref } from 'vue'
import { appWindow, Theme } from '@tauri-apps/api/window'
import { ClipboardHelper, ClipboardEntry } from '../clipboardHelper'
import { invoke } from '@tauri-apps/api/tauri'
import { convertFileSrc } from '@tauri-apps/api/tauri'

const textInput = ref('')
const clipboardEntries = ref<ClipboardEntry[]>([])
const selectedIndex = ref(-1)
let isKeyboardSelection = ref(true)

function openSettings() {
    invoke("rs_invoke_open_settings");
}

const filterPasteList = computed(() => {
  return clipboardEntries.value.map(item => {
    if (item.type === 0) {
      return item
    }
    const fileName = item.content.split('/').pop()
    return {
      ...item,
      content: fileName,
    }
  })
})

const displayContent = computed(() => {
  if (
    selectedIndex.value >= 0 &&
    selectedIndex.value < filterPasteList.value.length
  ) {
    return filterPasteList.value[selectedIndex.value].content
  }
  return ''
})

const selectedEntry = computed(() => {
  if (
    selectedIndex.value >= 0 &&
    selectedIndex.value < clipboardEntries.value.length
  ) {
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
    clipboardEntries.value = await ClipboardHelper.getClipboardEntries()
    selectedIndex.value = -1 // Reset selection
  } catch (error) {
    console.error('Failed to get clipboard content:', error)
    clipboardEntries.value = []
  }
}

async function searchClipboard() {
  try {
    clipboardEntries.value = await ClipboardHelper.searchClipboardEntries(
      textInput.value
    )
    selectedIndex.value = -1 // Reset selection
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
    } else if (
      e.key === 'ArrowDown' &&
      selectedIndex.value < clipboardEntries.value.length - 1
    ) {
      selectedIndex.value++
    }
  } else if (e.key === 'Enter' || ((e.metaKey || e.ctrlKey) && e.key === 'c')) {
    e.preventDefault() // 阻止默认的复制操作
    if (selectedIndex.value !== -1) {
      const selectedItem = clipboardEntries.value[selectedIndex.value]
      copyToClipboardAndHide(selectedItem)
    }
  } else if (e.key === 'Escape') {
    appWindow.hide()
  } else if ((e.metaKey || e.ctrlKey) && e.key === ',') {
    e.preventDefault() // 阻止默认行为
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
  const themeValue = await appWindow.theme()
  if (themeValue) {
    theme.value = themeValue
  }
}

onMounted(async () => {
  await getClipboardContent()
  document.addEventListener('keydown', handleKeydown)
  document.addEventListener('mousemove', handleMouseMove)

  await appWindow.onFocusChanged(async ({ payload: focused }) => {
    if (focused) {
      textInput.value = ''
      getClipboardContent()
      inputRef.value?.focus()
    }
    updateTheme()
  })

  inputRef.value?.focus()

  updateTheme()
})

watch(textInput, () => {
  if (textInput.value.trim() !== '') {
    searchClipboard()
  } else {
    getClipboardContent()
  }
})

const selectedTimestamp = computed(() => {
  if (
    selectedIndex.value >= 0 &&
    selectedIndex.value < clipboardEntries.value.length
  ) {
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
    default:
      return '📝'
  }
})

const handleSelectPasteItem = (index: number, item: any) => {
  selectedIndex.value = index
  copyToClipboardAndHide(item)
}

const hoverSettings = ref(false)
</script>

<template>
  <div
    class="main"
    :class="{
      'main-dark': theme === 'dark',
      'main-light': theme === 'light',
    }"
    data-tauri-drag-region
  >
    <div class="paste-filter">
      <input class="paste-filter-input" ref="inputRef" v-model="textInput" />
    </div>
    <div class="paste-content">
      <div class="paste-content-list">
        <div
          class="paste-content-item"
          :class="{
            'paste-content-item-selected': selectedIndex === index,
          }"
          v-for="(item, index) in filterPasteList"
          :key="item.id"
          @mouseover="
            () => {
              selectedIndex = index
            }
          "
          @click="handleSelectPasteItem(index, item)"
        >
          <div class="paste-item-icon">
            {{ pasteItemIcon(item.type) }}
          </div>
          <div class="paste-item-text">
            {{ item.content }}
          </div>
          <div class="paste-item-shortcut"></div>
        </div>
      </div>
      <div class="paste-content-desc">
        <div class="desc-wrapper">
          <img v-if="isImageEntry" :src="imageSrc" alt="Clipboard image" />
          <pre v-else>{{ displayContent }}</pre>
        </div>
        <div class="timestamp-wrapper" data-tauri-drag-region>
          <p
            class="timestamp-content"
            :class="{
              'timestamp-content-light': theme === 'light',
              'timestamp-content-dark': theme === 'dark',
            }"
          >
            <span v-if="selectedTimestamp">
              {{ formattedTimestamp }}
            </span>
            <span v-else> 输入值筛选剪贴板内容 </span>
          </p>
        </div>
      </div>
    </div>
    <div
      class="paste-settings"
      @mouseenter="
        () => {
          hoverSettings = true
        }
      "
      @mouseleave="
        () => {
          hoverSettings = false
        }
      "
    >
      <img
        v-if="theme === 'dark' || hoverSettings"
        class="paste-settings-icon paste-settings-icon-normal"
        src="../assets/settings-hover.svg"
        alt="Settings"
        @click="openSettings"
      />
      <img
        v-else
        class="paste-settings-icon paste-settings-icon-hover"
        src="../assets/settings.svg"
        alt="Settings"
        @click="openSettings"
      />
    </div>
  </div>
</template>

<style>
.main {
  width: 100%;
  height: 100vh;
  padding: 15px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  position: relative;
}
.main-light {
  color: #000;
}
.main-dark {
  color: #fff;
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
  background: rgba(0, 0, 0, 0.1);
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
  /* 自动外边距实现水平居中 */
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
  color: #3c3a3a;
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
</style>
