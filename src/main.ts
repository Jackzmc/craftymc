import { createApp } from 'vue'
import App from './App.vue'

import '@/assets/main.scss'
import { library } from "@fortawesome/fontawesome-svg-core";
import { faPlus } from "@fortawesome/free-solid-svg-icons";
import { faGithub } from '@fortawesome/free-brands-svg-icons'
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

library.add(faPlus, faGithub);

createApp(App)
  .component("fa-icon", FontAwesomeIcon)
  .mount('#app')
