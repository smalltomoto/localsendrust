import { defineStore } from "pinia";
import { ref } from "vue";
const initState = { id: 0, name: "", ip: "", online: true };
export const useDeviceStore = defineStore("device", () => {
    const selectedDevice = ref<Device>({ ...initState });

    const setSelectedDevice = (device: Device) => {
        selectedDevice.value = device
    }
    return {
        selectedDevice,
        setSelectedDevice
    }
}
);

// use
// import useDeviceStore
// const deviceStore = useDeviceStore()
// deviceStore.selectedDevice
// deviceStore.setSelectedDevice(inputParam)