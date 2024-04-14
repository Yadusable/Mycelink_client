<script lang="ts">
    export let showCreateAccountModal:boolean= false;
    import {fade} from "svelte/transition";

    let dialog:HTMLDialogElement;
    let form:HTMLFormElement;

    $: if (dialog && showCreateAccountModal) {
        dialog.showModal()
    } else if (dialog && !showCreateAccountModal) {
        dialog.close()
        showCreateAccountModal = false
    }

    function submitForm() {
        console.log(form.elements.namedItem("arc")?.value)
        console.log(form.elements.namedItem("dn")?.value)
        showCreateAccountModal=false
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
        display: grid;
        height: 19rem;
        width: 18rem;
        grid-template-rows: 1fr;
        grid-template-columns: 1fr;
        transition: opacity 0.3s ease;
    }

    .formGrid{
        display: grid;
        justify-items: center;
        grid-row: 1;
        grid-column: 1;
        grid-template-rows: 2fr 2fr 2fr 2fr 0.5fr 2fr;
        grid-auto-columns: auto;
        gap: 10px 0;
        height: 100%;
        width: 100%;
        transition: opacity 0.3s ease;
        min-height: 0; /*WHAT IN THE EVERLIVING ...*/

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
        /*grid-row: 3;*/
        grid-column: 1;
        width: 80%;
        box-sizing: border-box;
    }

    .buttons{
        /*width: 80%;*/
        grid-row: -2;
        grid-column: 1;
        justify-content: end;
        /*margin: 0 0 0 auto;*/
    }

    .createButton{
        background: var(--nice-teal);
        cursor: pointer;
    }
</style>

<dialog
        class="createModal"
        bind:this={dialog}
        on:close={() => (showCreateAccountModal = false)}
        transition:fade

>
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <form class="formGrid" on:submit|preventDefault={submitForm} method="dialog" bind:this={form}>
        <h1 class="title">Add a Contact</h1>
        <select class="selectMessenger">
            <option>Mycelink</option>
        </select>
        <input class="messengerSpecificContent" placeholder="Account Request Key" id="arc" name="arc">
        <input class="messengerSpecificContent" placeholder="Display Name" id="dn">
        <!-- svelte-ignore a11y-autofocus -->
        <div class="buttons">
            <button autofocus on:click={() => showCreateAccountModal = false}>cancel</button>
            <input type="submit" class="createButton" value="Create"/>
        </div>
    </form>
</dialog>