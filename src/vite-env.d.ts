/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

interface ElectronAPI {
  checkRust?: () => Promise<boolean>
  checkNode?: () => Promise<boolean>
  checkNpm?: () => Promise<boolean>
  checkVSBuildTools?: () => Promise<boolean>
  checkWebView2?: () => Promise<boolean>
}

declare global {
  interface Window {
    electron?: ElectronAPI
  }
}

export {}
