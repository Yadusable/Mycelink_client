<script lang="ts">
    import {emit} from "@tauri-apps/api/event";
    import ContactsBanner from "./ContactsBanner.svelte";

    let contacts = [
        { id: 1, display_name: 'John', alt_name: 'John_alt_name',profile_picture: '/mockfiles/profile_1.jpg', last_message: 'Hello there!' },
        { id: 2, display_name: 'Alice', alt_name: 'Alice_alt_name', profile_picture: '/mockfiles/profile_4.jpg', last_message: 'How are you?' },
        { id: 3, display_name: 'Bob', alt_name: 'Bob_alt_name', profile_picture: '/mockfiles/profile_2.jpg', last_message: '🖼 Image' },
        { id: 4, display_name: 'Clark', alt_name: 'Clark_alt_name', profile_picture: '/mockfiles/profile_7.jpg', last_message: 'HOGRIIIIIIDER' },
        { id: 5, display_name: 'Dick', alt_name: 'Dick_alt_name', profile_picture: '/mockfiles/profile_6.jpg', last_message: 'Please leave me alone jonjasjndasud' },
        { id: 6, display_name: 'Emily', alt_name: 'Emily_alt_name',profile_picture: '/mockfiles/profile_5.jpg', last_message: 'Pizza time!' },
        // Add more contacts as needed
    ];

    interface contactInterface {
        id: number;
        display_name: string;
        alt_name: string;
        profile_picture: string;
        last_message: string;
    }

     let selectedContact: contactInterface;

    export { contacts };
    export type {contactInterface};

    async function onSelectContact(contact: contactInterface) {
        selectedContact = contact
        await emit('contact-selected', selectedContact)
    }
</script>

<style>
    :root{
        --image-size: 100%;
        --image-margin-right: 15px;
        --selection-border-image: linear-gradient(135deg, #44bba4, #44bb74);

        --light-grey: #e7e5df;
        --medium-grey: #d3d0cb;
        --dark-grey: #5e686e;
        /*grid-column: 1;*/
        /*grid-row-start: 2;*/
        /*grid-row-end: 3;*/
    }
    .contact-item {
        height: 80px;
        width: 98%;
        max-width: 300px;
        display: flex;
        align-items: center;
        cursor: pointer;
        background-image: var(--selection-border-image);
        border: 4px solid var(--medium-grey);
        border-radius: 6px;
        overflow: hidden;
        padding: 0;
        margin-bottom: 7px;
        box-shadow: none;
    }
    .button-hover-background {
        background: #d3d0cb;
        padding: 7px 7px 7px 0;
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        overflow: hidden;
    }


    .contact-item:hover {
        border: 4px solid transparent;
        border-radius: 6px;
    }

    .profile-picture {
        /*width: var(--image-size);*/
        height: calc(var(--image-size) - 5px);
        aspect-ratio: 1;
        border-radius: 10%;
        margin: 10px var(--image-margin-right) 10px 5px;

    }

    .contact-info {
        display: flex;
        flex-direction: column;
        /*width: calc(100% - var(--image-size) - var(--image-margin-right));*/

    }

    .contact-name {
        font-family: Arial,sans-serif;
        font-size: 20px;
        font-weight: bold;
        line-height: 36px;
        height: 36px;
        width: 100%;
        text-align: left;
        color: #2f3437;
    }

    .contact-display-last-message {
        font-size: 13px;
        color: #888;
        line-height: 36px;
        height: 36px;
        text-align: left;
        text-overflow: ellipsis;
        overflow: hidden;
        white-space: nowrap;
        width: 100%;
    }

    .contact-list{
        overflow-y: scroll;
        /*height: 95%;*/
        /*scrollbar-gutter: stable;*/
        /*scrollbar-width: thin;*/
        /*scrollbar-color: red transparent;*/
        grid-row-start: 2;
        grid-row-end: -1;
        background: var(--dark-grey);
    }

    ::-webkit-scrollbar-track {
        background: transparent;
    }
    ::-webkit-scrollbar {
        /*display: none;*/
        width: 5px;
    }
    ::-webkit-scrollbar-thumb {
        background: #888;
        border-radius: 20px;
    }
    ::-webkit-scrollbar-thumb:hover {
        background: #777;
    }
    ::-webkit-scrollbar-thumb:active {
        background: #555;
    }
    ::-webkit-scrollbar:hover{
        display: compact;
    }
    ::-webkit-scrollbar-track-piece:hover{
        background: #0f0f0f;
    }
</style>


<ContactsBanner/>


<div class="contact-list">
    {#each contacts as contact (contact.id)}
        <button type="button" class="contact-item" on:click={() => onSelectContact(contact)}>
            <div class="button-hover-background">
                <img src={contact.profile_picture} alt={contact.display_name} class="profile-picture" />
                <div class="contact-info">
                    <div class="contact-name">{contact.display_name}</div>
                    <div class="contact-display-last-message">{contact.last_message}</div>
                </div>
            </div>
        </button>
    {/each}
</div>
