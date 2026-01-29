import {useNotifications} from "../components/Notification.tsx";
import {useRef} from "react";

export function useWasm() {
    let supported = useRef<boolean | null>(null);

    if (!supported.current) {
        supported.current = (() => {
            try {
                if (typeof WebAssembly === "object"
                    && typeof WebAssembly.instantiate === "function") {
                    const module = new WebAssembly.Module(Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00));
                    if (module instanceof WebAssembly.Module)
                        return new WebAssembly.Instance(module) instanceof WebAssembly.Instance;
                }
            } catch (e) {
            }
            return false;
        })();
    }

    const notifications = useNotifications();

    if (!supported.current) {
        notifications.error("Web assembly not supported! This simulation will not function at all.");
    }

    return supported;
}