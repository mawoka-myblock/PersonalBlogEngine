<script lang="ts">
    import { loggedIn, setupCompleted } from "./lib/stores";
    import Spinner from "./lib/components/Spinner.svelte";
    import MainScreen from "./lib/components/MainScreen.svelte";

    let checkingIfLoggedIn = true;
    const checkIfLoggedIn = async () => {
        const res = await fetch("/api/v1/account/check");
        if (res.status === 200) {
            loggedIn.set(true);
            setupCompleted.set(true);
        } else {
            loggedIn.set(false);
            const res2 = await fetch("/api/v1/manage/setup");
            if (res2.status === 200) {
                setupCompleted.set(true);
            } else {
                setupCompleted.set(false);
            }
        }
        checkingIfLoggedIn = false;
    };
    checkIfLoggedIn();
</script>

{#if checkingIfLoggedIn}
    <Spinner />
{:else if $loggedIn}
    <!-- Show Main-Menu -->
    <MainScreen />
{:else if $setupCompleted}
    <!-- Show Login-screen -->
    {#await import("./lib/components/Login.svelte") then c}
        <svelte:component this={c.default} />
    {/await}
{:else}
    <!-- Show Setup-screen -->
    <h1 class="text-center text-2xl">Setup</h1>
    {#await import("./lib/components/SetupScreen.svelte") then c}
        <svelte:component this={c.default} />
    {/await}
{/if}
