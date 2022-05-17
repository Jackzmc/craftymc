import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

import '@/assets/main.scss'
import { library } from "@fortawesome/fontawesome-svg-core";
import { faPlus, faCube, faCircleCheck, faCircle, faSearch, faHome, faSliders, faArrowUpRightFromSquare, faBars, faPlay, faCog } from "@fortawesome/free-solid-svg-icons";
import { faGithub } from '@fortawesome/free-brands-svg-icons'
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

// Only seems to work here
import '@creativebulma/bulma-tooltip/dist/bulma-tooltip.min.css'
import 'bulma-slider/dist/css/bulma-slider.min.css'

import Icon from '@/components/Icon.vue'

library.add(faPlus, faCube, faGithub, faCircleCheck, faCircle, faSearch, faHome, faSliders, faArrowUpRightFromSquare, faBars, faPlay, faCog);

createApp(App)
  .use(router)
  .component("fa-icon", FontAwesomeIcon)
  .component("Icon", Icon)
  .mount('#app')
