<script lang="ts">

    import CreateTenantModal from "./CreateTenantModal.svelte";

    export let showAddContactModal:boolean= false;
    import CreateAccountModal from "./CreateAccountModal.svelte"
    import {fade} from "svelte/transition";

    let dialog:HTMLDialogElement;
    let form:HTMLFormElement

    let showCreateTenantModal = false
    let tenants = ["User 1", "User 2", "User 3", "User 4", "User 5", "User 6"]

    $: if (dialog && showAddContactModal) {
        dialog.showModal()
    } else if (dialog && !showAddContactModal) {
        dialog.close()
        showAddContactModal = false
    }


    function nix(name:string) {
        console.log(name)
    }
</script>

<style>
    dialog.createModal {
        opacity: 0;
        pointer-events: none;
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        margin-top: 5px;
    }

    dialog[open].createModal {
        opacity: 1;
        pointer-events: auto;
    }
    .createModal{
        border-radius: 10px;
        height: 19rem;
        width: 18rem;
        transition: opacity 0.3s ease;
        display: grid;
        grid-template-rows: 2fr 4fr 0.5fr 2fr;
        grid-auto-columns: auto;
        gap: 10px 0;
        justify-items: center;
    }

    .accSelectGrid{
        display: flex;
        flex-wrap: wrap;
        width: 100%;
        overflow-y: scroll;
        /*display: grid;*/
        /*grid-row: 2;*/
        /*grid-template-rows: 2fr 2fr 2fr 2fr 0.5fr 2fr;*/
        /*grid-auto-columns: auto;*/
        /*gap: 10px 0;*/
        /*justify-items: center;*/
        height: 100%;
        min-height: 0;
    }

    .title{
        grid-row: 1;
        grid-column: 1;
    }

    .selectMessenger{
        grid-row: 2;
        grid-column: 1;
        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        font-size: 16px;
        width: 80%;

        /*background-color: #44bb74;*/
        border: none;
    }


    .messengerSpecificContent{
        grid-column: 1;
        width: 80%;
        box-sizing: border-box;
    }

    .buttons{
        display: flex;
        width: 100%;
        grid-row: -2;
        grid-column: 1;
        justify-content: end;
        margin: 0 auto;
        /*margin: 0 0 0 auto;*/
    }

    .accountButton{
        height: 40px;
        width: 70%;
        background-color: var(--medium-grey);
        color: #0f0f0f;
        margin: 3px auto;
    }

    .accountButton:hover{
        background-color: var(--nice-teal);
        color: var(--light-grey);
    }

    .wideButton{
        background: var(--dark-grey);
        width: 50%;
        height: 40px;
        margin: auto auto 0;


    }

    .wideButton:hover{
        cursor: pointer;
    }
</style>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
        class="createModal"
        bind:this={dialog}
        on:close={() => (showAddContactModal = false)}
        transition:fade

>
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <h1 class="title">Select an Account</h1>
    <div class="accSelectGrid">
        {#each tenants as tenant}
            <button on:click|preventDefault={() => nix(tenant)} class="accountButton">{tenant}</button>
        {/each}
    </div>
    <button class="accountButton" on:click={
            () => {
                showAddContactModal = false;
                showCreateTenantModal = true
            }
        }>
        Create new Account
    </button>
    <!-- svelte-ignore a11y-autofocus -->
    <div class="buttons">
        <button class="wideButton" autofocus on:click={() => showAddContactModal = false}>cancel</button>
    </div>
</dialog>
<div id="modals 2">
    <CreateTenantModal bind:showCreateTenantModal></CreateTenantModal>
</div>
