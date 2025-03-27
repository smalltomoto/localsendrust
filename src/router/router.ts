import { createRouter, createMemoryHistory } from 'vue-router';
import DeviceList from '../components/DeviceList.vue';
import SendFile from '../components/SendFile.vue';
import ReceiveFile from '../components/ReceiveFile.vue';
import History from '../components/History.vue';
import Setting from '../components/Setting.vue';

const routes = [
    { path: '/', redirect: '/deviceList', },
    { path: '/deviceList', component: DeviceList },
    { path: '/sendFile', component: SendFile },
    { path: '/receiveFile', component: ReceiveFile },
    { path: '/history', component: History },
    { path: '/setting', component: Setting }
];

const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

export default router;