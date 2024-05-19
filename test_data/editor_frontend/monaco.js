import * as Y from 'yjs'
import { WebsocketProvider } from 'y-websocket'
import { MonacoBinding } from 'y-monaco'
import * as monaco from 'monaco-editor'

// root domain of the api
const DOMAIN = "localhost:3000";

window.addEventListener('load', () => {
    const ydoc = new Y.Doc()

    // generate fake question id and team id
    const ids = [1, 2];
    const question_id = ids[Math.floor(Math.random() * ids.length)].toString();
    const team_id = ids[Math.floor(Math.random() * ids.length)].toString();

    const provider = new WebsocketProvider(
        `ws://${DOMAIN}/editor/socket`,
        `${question_id}/${team_id}`,
        ydoc
    )

    const ytext = ydoc.getText('monaco')

    const editor = monaco.editor.create((document.getElementById('monaco-editor')), {
        value: '',
        language: 'javascript',
        theme: 'vs-dark'
    })
    const monacoBinding = new MonacoBinding(ytext, (editor.getModel()), new Set([editor]), provider.awareness)

    const connectBtn = (document.getElementById('y-connect-btn'))
    connectBtn.addEventListener('click', () => {
        if (provider.shouldConnect) {
            provider.disconnect()
            connectBtn.textContent = 'Connect'
        } else {
            provider.connect()
            connectBtn.textContent = 'Disconnect'
        }
    })

    // @ts-ignore
    window.example = { provider, ydoc, ytext, monacoBinding }
})
