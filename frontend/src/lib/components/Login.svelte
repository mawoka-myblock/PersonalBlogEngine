<script lang="ts">
    import {loggedIn} from "../stores";

    let loginData = {
        email: "",
        password: ""
    };
    const logIn = async () => {
        const res = await fetch("/api/v1/account/login", {
            method: "POST",
            body: JSON.stringify({
                email: loginData.email,
                password: loginData.password
            }),
            headers: {
                "Content-Type": "application/json"
            }
        })
        if (res.status === 200) {
            loggedIn.set(true);
        }
    }

</script>

<div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4 flex flex-col">
    <div class="mb-4">
        <label class="block text-gray-800 text-sm font-bold mb-2" for="username">
            Username
        </label>
        <input class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-800" id="username" type="text"
               bind:value={loginData.email}
               placeholder="Username">
    </div>
    <div class="mb-6">
        <label class="block text-grey-darker text-sm font-bold mb-2" for="password">
            Password
        </label>
        <input class="shadow appearance-none border border-red rounded w-full py-2 px-3 text-gray-800 mb-3"
               bind:value={loginData.password}
               id="password" type="password" placeholder="******************">
        <!--        <p class="text-red text-xs italic">Please choose a password.</p>-->
    </div>
    <div class="flex items-center justify-between">
        <button class="bg-blue-700 hover:bg-blue-dark text-white font-bold py-2 px-4 rounded" type="button" on:click={logIn}>
            Sign In
        </button>
    </div>
</div>