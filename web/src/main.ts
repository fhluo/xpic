import './app.css'
import './i18n'
import App from './app.svelte'
import {mount} from "svelte";

const app = mount(App, {
    target: document.getElementById('app')!
})

export default app
