import { io } from 'socket.io-client';

// "undefined" means the URL will be computed from the `window.location` object
// const URL = process.env.NODE_ENV === 'production' ? undefined : 'http://localhost:3003';

const URL = 'https://a55b-49-36-83-210.ngrok-free.app';

export const newSocket = io(URL, {
    query: {
        token: '',
        nickName: 'NodeId 1'
    },
    transports: ["websocket"]
});