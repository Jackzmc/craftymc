export interface AppSettings {
  general: GeneralSettings,
  minecraft: MinecraftSettings,
  meta: AppMeta
}

export interface AppMeta {
  maxMemoryMb: number
}

// Needs to be flat to tie to rust struct
export interface GeneralSettings {

}
// Needs to be flat to tie to rust struct
export interface MinecraftSettings {
  saveDirectory: string,
  preferredRelease: string
  width: number,
  height: number,
  javaMemoryMb: number,
  javaPath: string | null,
  javaArgs: string | null
}
