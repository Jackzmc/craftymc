export interface ModrinthProject {
  id: string
  slug: string
  project_type: string
  team: string
  title: string
  description: string
  body: string
  body_url: string
  published: string
  updated: string
  status: string
  moderator_message: any
  license: {
    id: string
    platform: string
    url: string
  }
  client_side: string
  server_side: string
  downloads: number
  followers: number
  categories: string[]
  versions: string[]
  icon_url: string
  issues_url: string
  source_url: string
  wiki_url: any
  discord_url: string
  donation_urls: {
    id: string
    name: string
    url: string
  }
  gallery: any[]
}

export type ExternalProject = ModrinthProject
