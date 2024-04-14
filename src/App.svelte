<!-- App.svelte -->
<script>
  import ContactList from './Chatselectionframe.svelte';
  import ChatArea from './Chatframe.svelte';
  // import { invoke } from '@tauri-apps/api/tauri';
</script>

<div class="app-container">
  <ContactList />
  <ChatArea />
</div>

<style>
  .app-container {
    display: grid;
    grid-template-columns: minmax(200px, 280px) ;
    grid-template-rows: minmax(50px, 40px) auto minmax(50px, 10%);
    overflow: clip;
    background: #e7e5df;
    height: 100vh;
    width: 100vw;
  }

  /* AAAAAARRRRRGGGGGHHHHHH wtf*/
  :global(body) {
    margin: 0;
  }
</style>
