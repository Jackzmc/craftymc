export interface Modpack {
  id: string,
  name: string,
  versions: {
    minecraft: string,
    modloader: string,
    pack?: string
  },
  settings: PackSettings
}

export interface PackSettings {
  modloaderType: string,
  javaMemory?: number,
  useCustomMemory: boolean
}

export const enum View {
  Main,
  MyPacks,
  BrowsePacks,
  Settings
}

export const enum InstallState {
  Installed,
  NotInstalled,
  Installing,
}
