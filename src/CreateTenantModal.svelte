<script lang="ts">
    export let showCreateTenantModal:boolean= false;
    import {fade} from "svelte/transition";

    let dialog:HTMLDialogElement;
    let form:HTMLFormElement;

    $: if (dialog && showCreateTenantModal) {
        dialog.showModal()
    } else if (dialog && !showCreateTenantModal) {
        dialog.close()
        showCreateTenantModal = false
    } else {
    }

    function submitForm() {
        console.log(form.elements.namedItem("name")?.value)
        showCreateTenantModal=false
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

        grid-template-rows: 2fr 2fr 2fr 0.5fr 2fr;
        grid-auto-columns: auto;
        gap: 10px 0;
        justify-items: center;
    }

    .formGrid{
        display: grid;
        grid-template-rows: 2fr 2fr 2fr 2fr 0.5fr 2fr;
        grid-auto-columns: auto;
        gap: 10px 0;
        justify-items: center;
        height: 100%;
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
        /*width: 80%;*/
        grid-row: -1;
        grid-column: 1;
        justify-content: end;
        /*margin: 0 0 0 auto;*/
    }

    .createButton{
        background: var(--nice-teal);
    }

    .createButton:hover{
        cursor: pointer;
    }
</style>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions-->
<dialog
        class="createModal"
        bind:this={dialog}
        on:close={() => (showCreateTenantModal = false)}
        transition:fade

>
<!-- svelte-ignore a11y-no-static-element-interactions -->
    <h1 class="title">Create a new user</h1>
    <select class="selectMessenger">
        <option>Mycelink</option>
    </select>
    <input class="messengerSpecificContent" placeholder="Your Display Name">
    <!-- svelte-ignore a11y-autofocus -->
    <div class="buttons">
    <button autofocus on:click={() => showCreateTenantModal = false}>cancel</button>
    <button class="createButton">create</button>
    </div>
</dialog>