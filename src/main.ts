import { createApp } from 'vue'
import App from './App.vue'

import '@/assets/main.scss'
import { library } from "@fortawesome/fontawesome-svg-core";
import { faPlus, faCube, faCircleCheck, faCircle, faSearch, faHome, faSliders, faArrowUpRightFromSquare } from "@fortawesome/free-solid-svg-icons";
import { faGithub } from '@fortawesome/free-brands-svg-icons'
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

import '@creativebulma/bulma-tooltip/dist/bulma-tooltip.min.css'

library.add(faPlus, faCube, faGithub, faCircleCheck, faCircle, faSearch, faHome, faSliders, faArrowUpRightFromSquare);

createApp(App)
  .component("fa-icon", FontAwesomeIcon)
  .mount('#app')
