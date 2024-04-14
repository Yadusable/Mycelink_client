<script>
    import CreateAccountModal from "./CreateAccountModal.svelte";
    import AddContactModal from "./AddContactModal.svelte";

    let showCreateAccountModal = false;
    let showAddContactModal = false

    function createAccount() {
        console.log("Create Account button was clicked")
        showCreateAccountModal = true
    }

    function addContact() {
        console.log("Contact add button was clicked")
        showAddContactModal = true
    }
</script>

<style>

    :root{
        --light-grey: #e7e5df;
        --medium-grey: #d3d0cb;
        --dark-grey: #5e686e;
    }

    .banner {
        display: grid;
        /*grid-template-rows: 50% 50% ;*/
        grid-template-columns: 1fr 45px 45px 0;
        overflow: clip;
        background: var(--dark-grey);
        height: 100%;
        /*width: 100%;*/
        grid-auto-flow: row;
        grid-template-areas:
            "whitespace add-chat more-options";
        gap: 0 10px;
        box-sizing: border-box;
        grid-column: 1;
        grid-row: 1;
    }

    .svg-sign {
        display: flex;
        align-items: center;
        margin: auto;
        /*height: 40%;*/
        width: 50%;
        fill: #e7e5df;

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
        box-shadow: none;
        /*grid-row: 1 /span 2;*/
        aspect-ratio: 1;
    }

    .invisible-button:hover {
        background: #888888;
    }


    .text {
        text-align: left;
        margin-left: 8px;
        margin-bottom: 0;
        margin-top: 0;
        padding-top: 20px;
    }

    .modals {
        opacity: 0;
    }
</style>

<div class="banner">
    <h1 class="text"> Chats</h1>
    <button class="invisible-button" on:click={createAccount}>
        <img src="/addcontact.svg" class="svg-sign">
<!--        <svg class="svg-sign" viewBox="0 0 12 13" height="40%" >-->
<!--            <g stroke-width="2" stroke="#e7e5df" fill="none"><path d="M11.29 11.71l-4-4"/><circle cx="5" cy="5" r="4"/>-->
<!--            </g>-->
<!--        </svg>-->
    </button>
    <button class="invisible-button" on:click={addContact}>
        <svg class="svg-sign" viewBox="0 0 378 100" height="13%">
            <path stroke="#e7e5df"
                  d="m 94.344824,47.172415 c 0,14.151724 -4.717246,23.586209 -14.151724,33.020697 -9.434485,9.434478 -21.227591,14.151724 -33.020691,14.151724 -14.15173,0 -23.58621,-4.717246 -33.02069,-14.151724 C 4.7172294,70.758624 -5.9265128e-7,58.965516 -5.9265128e-7,47.172415 -5.9265128e-7,35.379314 4.7172294,23.586207 14.151719,14.151724 23.586199,4.717242 35.379299,0 47.172409,0 c 11.7931,0 23.586206,4.717242 33.020691,14.151724 9.434478,9.434483 14.151724,21.22759 14.151724,33.020691 z m 141.517246,0 c 0,14.151724 -4.71724,23.586209 -14.15172,33.020697 -9.43449,9.434478 -21.2276,14.151724 -33.0207,14.151724 -14.15172,0 -23.58621,-4.717246 -33.02069,-14.151724 -9.43448,-9.434488 -14.15172,-21.227596 -14.15172,-33.020697 0,-11.793101 4.71724,-23.586208 14.15172,-33.020691 C 165.10344,4.717242 176.89655,0 188.68965,0 c 11.7931,0 23.58621,4.717242 33.0207,14.151724 9.43448,9.434483 14.15172,21.22759 14.15172,33.020691 z m 141.51724,0 c 0,14.151724 -4.71724,23.586209 -14.15172,33.020697 C 353.79311,89.62759 342,94.344836 330.2069,94.344836 c -14.15172,0 -23.58621,-4.717246 -33.02069,-14.151724 -9.43448,-9.434488 -14.15173,-21.227596 -14.15173,-33.020697 0,-11.793101 4.71725,-23.586208 14.15173,-33.020691 C 306.62069,4.717242 318.41379,0 330.2069,0 342,0 353.79311,4.717242 363.22759,14.151724 c 9.43448,9.434483 14.15172,21.22759 14.15172,33.020691 z" />
        </svg>
    </button>
</div>
<CreateAccountModal bind:showCreateAccountModal></CreateAccountModal>
<AddContactModal bind:showAddContactModal></AddContactModal>

