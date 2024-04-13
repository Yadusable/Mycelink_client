<style>
    :root{
        --button-size: 26px;
        --chat-info-height: 4%;
        --new-message-area-height: 40px;
        --recieved-message-left-margin: 15px;
        --sentmessage-right-margin: 20px;
        --textmessage-margin: 10px;
        grid-column: 2;
        grid-row: 1;
        --light-grey: #e7e5df;
        --medium-grey: #d3d0cb;
        --dark-grey: #5e686e;
    }

    .grid-box {
        display: grid;
        grid-template-columns: 32px 1fr 32px;
        grid-template-rows: 1fr;
        column-gap: 10px;
        grid-auto-flow: row;
        justify-content: center;
        align-content: center;
        justify-items: center;
        align-items: center;
        border-radius: 10px;
        margin: 3px 10px 10px 10px;

        background: #d3d0cb;
        padding: 10px;
        grid-row: 3;
        grid-column: 2;
        grid-template-areas:
    "add-attachment-button new-message-textbox send-button";
    }

    .new-message-textbox {
        grid-area: new-message-text;
        background: #888888;
        width: 98%;
        border-radius: 5px;
        color: #0f0f0f;
        font-size: 20px;
        resize: none;
        border: transparent;
        justify-content: right;
        height: 100%;
        grid-row: 1;
        grid-column: 2;
        padding: 0 10px 0 10px;
        box-sizing: border-box;
        box-shadow: none;



        justify-items: center;
        align-items: center;
    }

    .send-button {
        grid-area: send-button;
        /*width: var(--button-size);*/
        /*height: var(--button-size);*/
        aspect-ratio: 1;
        padding: 0;
        /*height: var(--button-size);*/
        width: 100%;
        background-color: #6cb8a5;
        border-radius: 5px;
        /*margin-left: 10px;*/
        /*margin-right: 20px;*/


        grid-row: 1;
        grid-column: 3;

        justify-content: right;
        align-content: center;
        justify-items: center;
        align-items: center;
        box-sizing: border-box;
        box-shadow: none;


    }

    .add-attachment-button {
        grid-area: add-attachment-button;
        background: var(--light-grey);
        border-radius: 50%;
        width: var(--button-size);
        height: var(--button-size);
        /*aspect-ratio: 1;*/
        grid-row: 1;
        /*padding-left: 20px;*/
        grid-column: 1;

        justify-items: center;
        align-items: center;
        box-sizing: border-box;
    }

    .add-attachment-button:hover{
        cursor: pointer;
        border: 1px solid dodgerblue;

        width: calc(var(--button-size) - 2);
        height: calc(var(--button-size) - 2);
    }

    .plus-svg-sign {
        display: block;
        margin: auto;
    }
    input[type="file"] {
        display: none;
    }
</style>

<script lang="ts">
    import { invoke } from "@tauri-apps/api/core"
    let msg_text:string;
    let response_text:string;
    
    async function send_msg(){
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        response_text = await invoke("send_msg", { text:msg_text })
        console.log(response_text)
        msg_text=""
    }

    let files:FileList;
    $: if (files) {
        console.log(files);

        for (const file of files) {
            console.log(`${file.name}: ${file.size} bytes`);
            console.log(`${file.toString()}`)
        }
    }
</script>

<form class="grid-box" on:submit|preventDefault={send_msg}>
    <input bind:files multiple type="file" id="file_upload">
    <label for="file_upload">
            <svg class="plus-svg-sign add-attachment-button" viewBox="0 0 500 600" height="24" width="24" preserveAspectRatio="xMidYMid meet">
                <g>
                    <path fill-rule="evenodd" clip-rule="evenodd" d="M 241.99219,407.91016 L 241.99219,310.44922 L 144.72656,310.44922 L 144.72656,294.43359 L 241.99219,294.43359 L 241.99219,197.36328 L 257.61719,197.36328 L 257.61719,294.43359 L 355.27344,294.43359 L 355.27344,310.44922 L 257.61719,310.44922 L 257.61719,407.91016 L 241.99219,407.91016 z"/>
                </g>
            </svg>
    </label>
    <input class="new-message-textbox" contenteditable="true" placeholder="Start typing!" id ="text_input" bind:value={msg_text} autofocus/>
    <button class="send-button" type="submit"> &gt; </button>
</form>
