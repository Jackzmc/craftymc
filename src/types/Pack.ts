export interface Modpack {
  id: string,
  name: string,
  versions: {
    minecraft: string,
    forge: string,
    pack: string
  },
  settings: PackSettings
}

export interface PackSettings {
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
