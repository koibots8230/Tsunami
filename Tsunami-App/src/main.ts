import {App, createApp} from "vue";
import "./styles/main.scss";
import Tsunami from "./Tsunami.vue";

const app: App = createApp(Tsunami)
app.mount("#app");
