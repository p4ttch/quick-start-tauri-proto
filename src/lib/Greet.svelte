<script>
    import { invoke } from '@tauri-apps/api/tauri';

    let name = '';
    let greetMsg = '';

    async function greet() {
        greetMsg = await invoke('greet', { name });
    }
</script>

<div class="form-container">
    <div class="greeting-form">
        <input id="greet-input" placeholder="Enter a name..." bind:value="{name}">
        <button on:click="{greet}">Greet </button>
        </div>
    <p>{greetMsg}</p>
</div>

<style>
.form-container{
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    font-family: Arial, sans-serif;
    font-size: 1.5rem;
    color: #333;
    background-color: #ccc;
}

.greeting-form{
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    gap: 1rem;
}
</style>