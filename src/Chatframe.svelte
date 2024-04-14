<script lang="ts">
    import Newmessagearea from "./Newmessagearea.svelte";
    import ChatInfoBanner from "./ChatInfoBanner.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {listen} from "@tauri-apps/api/event";
    import contactInterface from "./Chatselectionframe.svelte"

    let localSelectedContact:contactInterface;
    let profile_picture


    let exampleMessages = [
        { messageId: 0o123, sender: {display_name: "Me", contact_id: 0o1}, messageType: {'text':{content: "Tschüss, du Arschloch!"}}, timestamp: 1709025924 },
        { messageId: 0o124, sender: {display_name: "You", contact_id: 0o2}, messageType: {'text':{content: "Entschuldigung?"}} , timestamp: 1709025924 },
        { messageId: 0o125, sender: {display_name: "Me", contact_id: 0o1}, messageType: {'text':{content: "Verzeihung."}} , timestamp: 1709025924 },
        { messageId: 0o126, sender: {display_name: "Me", contact_id: 0o1}, messageType: {'text':{content: "Tschüss, Sie Arschloch!"}}, timestamp: 1709025924 },
        { messageId: 0o123, sender: {display_name: "Me", contact_id: 0o1}, messageType: {'text':{content: "Tschüss, du Arschloch!"}}, timestamp: 1709025924 },
        { messageId: 0o124, sender: {display_name: "You", contact_id: 0o2}, messageType: {'text':{content: "Entschuldigung?"}} , timestamp: 1709025924 },
        { messageId: 0o125, sender: {display_name: "Me", contact_id: 0o1}, messageType: {'text':{content: "Verzeihung."}} , timestamp: 1709025924 },
        { messageId: 0o126, sender: {display_name: "Me", contact_id: 0o1}, messageType: {'text':{content: "Tschüss, Sie Arschloch!"}}, timestamp: 1709025924 },
        { messageId: 0o123, sender: {display_name: "Me", contact_id: 0o1}, messageType: {'text':{content: "Tschüss, du Arschloch!"}}, timestamp: 1709025924 },
        { messageId: 0o124, sender: {display_name: "You", contact_id: 0o2}, messageType: {'text':{content: "Entschuldigung?"}} , timestamp: 1709025924 },
        { messageId: 0o125, sender: {display_name: "Me", contact_id: 0o1}, messageType: {'text':{content: "Verzeihung."}} , timestamp: 1709025924 },
        { messageId: 0o126, sender: {display_name: "Me", contact_id: 0o1}, messageType: {'text':{content: "Tschüss, Sie Arschloch!"}}, timestamp: 1709025924 }
    ];

    function sendMessage(){
        const text = document.getElementById("text_input");
        console.log(text);
        console.log("ALARRRM");
    }

    listen('contact-selected', (event) => {
        localSelectedContact = event.payload as contactInterface
        profile_picture = localSelectedContact.profile_picture
    });

</script>


<style>
    :root{
        /*--button-size: 30px;*/
        /*--chat-info-height: 4%;*/
        /*--new-message-area-height: 40px;*/
        /*--recieved-message-left-margin: 15px;*/
        /*--sentmessage-right-margin: 20px;*/
        --light-grey: #e7e5df;
        --medium-grey: #d3d0cb;
        --dark-grey: #5e686e;
        --nice-teal: #6cb8a5;
    }

    .full-width{
        width: 100%;
        overflow: clip;
        /*height: 80%;*/
    }

    .chat-history-area {
        flex: 2;
        /*padding: 0 10px 0 10px;*/
        /*height: calc(100% - var(--chat-info-height) - var(--new-message-area-height) - 30px);*/
        width: 100%;
        /*margin-right: 10px;*/
        /*overflow-x: scroll;*/
        overflow-y: scroll;
        background: #e7e5df;
        grid-column: 2;
        grid-row: 2;
    }

    .chat-line {
        display: flex;
        padding-bottom: 2px;
        padding-top: 2px;
        }

    .chat-bubble {
        padding: 10px;
        text-align: center;
        border-radius: 18px;
    }

    .chat-info {
        /*width: 100%;*/
        padding-top: 10px;
        padding-bottom: 10px;
        background: #888888;
        justify-content: center;
        /*height: var(--chat-info-height);*/

    }

    .sent-message {
        background: var(--nice-teal);
        margin-right: 10px;
        border-bottom-right-radius: 5px;
    }

    .received-message {
        background: #d3d0cb;
        color: black;
        border-bottom-left-radius: 5px;
    }

    .time-sent {
        color: #60686d;
    }

    .message-status {
        justify-content: center;
        height: 15px;
        width: 30px;
        background: #0f0f0f;
    }

    .chat-line:has(.sent-message){
        justify-content: right;
        margin-right: 0;
    }

    .chat-line:has(.received-message){
        justify-content: left;
        margin-left: 15px;
    }

    &[placeholder]:empty:not(:focus):before {
        content: attr(placeholder);
    }

    .placeholder{
        grid-row: 2;
        grid-column: 2;
        text-align: center;
        color: var(--nice-teal );
        font-size: 24px;
        margin: auto;
    }

</style>
    <ChatInfoBanner/>
    {#if localSelectedContact}

        <div class="chat-history-area">
            {#each exampleMessages as message}
                {#if message.sender && message.sender.contact_id !== 0o1}
                    <div class="chat-line">
                        <div class="chat-bubble received-message">{message.messageType.text.content}</div>
                    </div>
                {:else if message.sender}
                    <div class="chat-line">
                        <div class="chat-bubble sent-message">{message.messageType.text.content}</div>
                    </div>
                {:else}
                    <!-- //Status-->
                    <div class="chat-line">
                        <div class="chat-info">{message.messageType.text.content}</div>
                    </div>
                {/if}
            {/each}
        </div>
        <Newmessagearea/>
    {:else}
        <div class="placeholder">Select a contact to start chatting.</div>
    {/if}
