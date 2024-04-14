<script lang="ts">
    import {listen} from '@tauri-apps/api/event'
    import contactInterface from "./Chatselectionframe.svelte"
    let selectedContact:contactInterface
    let displayName:string
    let profile_picture:string

    listen('contact-selected', (event) => {
        selectedContact = event.payload as contactInterface
        displayName = selectedContact.display_name
        profile_picture = selectedContact.profile_picture
    });
</script>

<style>
    .banner {
        display: grid;
        grid-template-rows: 50% 50% ;
        grid-template-columns: 60px 1fr 45px 45px 0;
        overflow: clip;
        background: #888888;
        height: 100%;
        /*width: 100%;*/
        grid-auto-flow: row;
        grid-template-areas:
            "pfp name search more-options"
            "pfp status search more-options";
        gap: 0 10px;
        box-sizing: border-box;
        grid-column: 2;
        grid-row: 1;
    }

    .svg-sign {
        display: flex;
        align-items: center;
        margin: auto;
        fill: #e7e5df;

    }

    .pfp {
        display: flex;
        /*align-items: center;*/
        grid-area: pfp;
        height: 80%;
        border-radius: 20%;
        margin-left: 10px;
        margin-top: 5px;
        grid-column: 1;
        grid-row: 1 /span 2;
        /*justify-content: center;*/
    }

    .chat-name {
        grid-area: name;
        grid-column: 2;
        grid-row: 1;
    }

    .chat-status {
        grid-area: status;
        grid-column: 2;
        grid-row: 2;
        height: 100%;
        margin: 0;
    }

    .invisible-button {
        background: none no-repeat;
        border: none;
        overflow: hidden;
        outline: none;
        height: calc(100% - 6px);
        padding: 0;
        margin-top: 3px;
        margin-bottom: 3px;
        aspect-ratio: 1;
        box-shadow: none;
        /*grid-row-start: 1;*/
        /*grid-row-end: 2;*/
        grid-row: 1 /span 2 ;
    }

    .invisible-button:hover {
        background: #44bb74;
    }

    .no_contact_selected{
        visibility: hidden;
    }
</style>

{#if selectedContact}
    <div class="banner">
        <img class="pfp" src={profile_picture} alt="">
        <b class="chat-name">{displayName}</b>
        <small class="chat-status">busy</small>
        <button class="invisible-button">
            <svg class="svg-sign" viewBox="0 0 12 13" height="40%">
                <g stroke-width="2" stroke="#e7e5df" fill="none"><path d="M11.29 11.71l-4-4"/><circle cx="5" cy="5" r="4"/>
                </g>
            </svg>
        </button>
        <button class="invisible-button">
            <svg class="svg-sign" viewBox="0 0 378 100" height="13%">
                <path stroke="#e7e5df"
                    d="m 94.344824,47.172415 c 0,14.151724 -4.717246,23.586209 -14.151724,33.020697 -9.434485,9.434478 -21.227591,14.151724 -33.020691,14.151724 -14.15173,0 -23.58621,-4.717246 -33.02069,-14.151724 C 4.7172294,70.758624 -5.9265128e-7,58.965516 -5.9265128e-7,47.172415 -5.9265128e-7,35.379314 4.7172294,23.586207 14.151719,14.151724 23.586199,4.717242 35.379299,0 47.172409,0 c 11.7931,0 23.586206,4.717242 33.020691,14.151724 9.434478,9.434483 14.151724,21.22759 14.151724,33.020691 z m 141.517246,0 c 0,14.151724 -4.71724,23.586209 -14.15172,33.020697 -9.43449,9.434478 -21.2276,14.151724 -33.0207,14.151724 -14.15172,0 -23.58621,-4.717246 -33.02069,-14.151724 -9.43448,-9.434488 -14.15172,-21.227596 -14.15172,-33.020697 0,-11.793101 4.71724,-23.586208 14.15172,-33.020691 C 165.10344,4.717242 176.89655,0 188.68965,0 c 11.7931,0 23.58621,4.717242 33.0207,14.151724 9.43448,9.434483 14.15172,21.22759 14.15172,33.020691 z m 141.51724,0 c 0,14.151724 -4.71724,23.586209 -14.15172,33.020697 C 353.79311,89.62759 342,94.344836 330.2069,94.344836 c -14.15172,0 -23.58621,-4.717246 -33.02069,-14.151724 -9.43448,-9.434488 -14.15173,-21.227596 -14.15173,-33.020697 0,-11.793101 4.71725,-23.586208 14.15173,-33.020691 C 306.62069,4.717242 318.41379,0 330.2069,0 342,0 353.79311,4.717242 363.22759,14.151724 c 9.43448,9.434483 14.15172,21.22759 14.15172,33.020691 z" />
            </svg>
        </button>
    </div>
{:else}
    <div class="no_contact_selected">
    </div>
{/if}
