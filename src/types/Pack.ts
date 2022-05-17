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

export enum View {
  Main,
  MyPacks,
  BrowsePacks,
  Settings
}
