<script lang="ts">
    import type { PageData } from "./$types";
    import CaptureTask from "$lib/CaptureTask.svelte";
    import { appWindow } from "@tauri-apps/api/window";
    import { sendNotification } from '@tauri-apps/api/notification';

    export let data: PageData;

    enum STATE {
        NEW,
        RUNNING,
        PAUSED,
        TIMEUP,
    }

    let ontopSetting: boolean = false;
    let state: STATE = STATE.NEW;
    let pomoTime: number = 0;
    let startTime: number = 0;
    let elapsedTime: number = 0;
    let oldElapsedTime: number = 0;
    let interval: number;
    let alertSound:string = "Alarm8"

    const pad2 = (number: number) => `00${number}`.slice(-2);
    const pad3 = (number: number) => `00${number}`.slice(-3);

    $: hours = pad2(Math.floor(elapsedTime / 1000 / 60 / 60) % 60);
    $: minutes = pad2(Math.floor(elapsedTime / 1000 / 60) % 60);
    $: seconds = pad2(Math.floor(elapsedTime / 1000) % 60);
    $: millis = pad3(elapsedTime % 1000);
    $: formattedElapsedTime = `${hours}:${minutes}:${seconds}.${millis}`;
    const start = () => {
        startTime = Date.now();
        state = STATE.RUNNING;
        interval = setInterval(() => {
            if (state === STATE.RUNNING) {
                const endTime = Date.now();
                console.log("endtime:", endTime, " starTime: ", startTime);
                elapsedTime = endTime - startTime + oldElapsedTime;
                if (elapsedTime > 1000) {
                    // console.log("times up");
                    // pause();
                }
            }
        });
    };

    const countdown = () => {
        startTime = Date.now() + pomoTime;
        state = STATE.RUNNING;
        interval = setInterval(() => {
            if (state === STATE.RUNNING) {
                const endTime = Date.now();
                console.log(
                    "endtime:",
                    endTime,
                    " starTime: ",
                    startTime,
                    " pomoTime: ",
                    pomoTime
                );
                // elapsedTime = pomoTime - endTime;
                elapsedTime = startTime - endTime;
                if (elapsedTime < 0) {
                    console.log("times up", elapsedTime);
                    pause();
                    elapsedTime = 0;
                    attention();
                    // trigger input
                    /** TODO:
                     * [ X ] call attention to user, set window to toplayer
                     * [ ] show an input field
                     * [ ] catch start time, end of session time, input capture time
                     * [ ] capture the data
                     * [ ] offer new counter
                     * 
                     * [ ] recall data
                     * [ ] display data on dashboard in table
                     * [ ] export to csv / excel file
                     */
                }
            }
        });
    };

    async function attention() {
        ontopSetting = !ontopSetting;
        await appWindow.setAlwaysOnTop(ontopSetting);
        await appWindow.setClosable(!ontopSetting);
        await appWindow.requestUserAttention(2);
        sendNotification({ title: 'Task Timer'+ontopSetting, body: 'Timer has ended, what have you been busy with?', sound:alertSound });
    }

    // TODO: use dispatch on form submit to control alwaysOnTop
    async function resetAttention() {
        ontopSetting = false;
        await appWindow.setAlwaysOnTop(ontopSetting);
        await appWindow.setClosable(!ontopSetting);
    }

    const reset = () => {
        elapsedTime = 0;
        oldElapsedTime = 0;
        state = STATE.NEW;
        clearInterval(interval);
        resetAttention();
    };

    const pause = () => {
        state = STATE.PAUSED;
        oldElapsedTime = elapsedTime;
    };

    const resume = () => {
        state = STATE.RUNNING;
        // startTime = Date.now();
    };

    const setPomo = () => {
        // pomoTime = 1500000; // 25 min
        elapsedTime = 1000;
        pomoTime = 1000; // 25 min
    };
</script>

<div class="flex flex-col justify-center items-center h-screen">
    <div class="glass w-2/3 h-26 p-4">
        <h1 class="text-2xl mb-2 text-yellow-100 border-b border-white">
            Super stop watch
        </h1>

        <p class="text-xl text-white mb-2">
            {formattedElapsedTime}
        </p>

        <div class="text-right">
            {#if state === STATE.NEW}
                <!-- <button
                    on:click={start}
                    class="text-xl mr-2 border-white border rounded text-green-300 px-2"
                    >Start</button
                > -->
                <button
                    on:click={countdown}
                    class="text-xl mr-2 border-white border rounded text-green-300 px-2"
                    >Count Down</button
                >
                <button
                    on:click={setPomo}
                    class="text-xl mr-2 border-white border rounded text-green-300 px-2"
                    >Set pomo</button
                >
            {/if}
            {#if state === STATE.RUNNING || state === STATE.PAUSED}
                <button
                    on:click={reset}
                    class="text-xl mr-2 border-red-300 border rounded text-red-300 px-2"
                    >Resest</button
                >
            {/if}
            {#if state === STATE.RUNNING}
                <button
                    on:click={pause}
                    class="text-xl mr-2 border-white border rounded text-white px-2"
                    >pause</button
                >
            {/if}
            {#if state === STATE.PAUSED}
                <button
                    on:click={resume}
                    class="text-xl border-black border rounded text-black px-2"
                    >Resume</button
                >
            {/if}
        </div>

        <CaptureTask />
    </div>
</div>

<style lang="postcss">
    .glass {
        background: rgba(255, 235, 233, 0.15);
        box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
        backdrop-filter: blur(5px);
        border-radius: 10px;
    }
</style>
