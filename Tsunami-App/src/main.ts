import {App, createApp} from "vue";
import "./styles.css";
import Tsunami from "./Tsunami.vue";

let i = 0;
const app: App = createApp(Tsunami)

setInterval(() => {
    console.log(i);
    i++;
},  
1000 ); // Start 60hz periodic function

app.mount("#app");