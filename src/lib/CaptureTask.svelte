<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";


    let name = "";
    let greetMsg = "";

    async function greet() {
        greetMsg = await invoke("greet", { name });
    }


</script>

<div>
    <div class="form-container">
        <h1 class="text-xl">What have you been busy with?</h1>
        <div class="greeting-form">
            <input
                id="greet-input"
                placeholder="Enter a name..."
                bind:value={name}
            />
            <button on:click={greet} class="w-28 bg-white shadow rounded"
                >Submit</button
            >
        </div>
        <p>{greetMsg}</p>
    </div>
</div>
