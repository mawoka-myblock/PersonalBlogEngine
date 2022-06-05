<script lang="ts">
    import * as yup from "yup";
    import { setupCompleted } from "../stores";

    let setupData = {
        email: "",
        password: "",
    };
    const registerSchema = yup.object({
        email: yup
            .string()
            .email("The email isn't valid.")
            .required("An email is required."),
        password: yup
            .string()
            .min(6, "The password must be at least 6 characters.")
            .required("A password is required."),
    });
    const completeSetup = async () => {
        if (!(await registerSchema.isValid(setupData))) {
            return;
        }
        const res = await fetch("/api/v1/manage/setup", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(setupData),
        });
        if (res.status === 200) {
            setupCompleted.set(true);
        }
    };
    let yupErrorMessage = "";
    let inputValid = false;
    const validateInput = async (data) => {
        try {
            await registerSchema.validate(data, { abortEarly: false });
            inputValid = true;
            yupErrorMessage = "";
        } catch (err) {
            inputValid = false;
            yupErrorMessage = err.errors[0];
        }
    };
    $: validateInput(setupData);
</script>

<div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4 flex flex-col">
    <div class="mb-4">
        <label class="block text-gray-800 text-sm font-bold mb-2" for="email">
            E-Mail
        </label>
        <input
            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-800"
            id="email"
            type="text"
            bind:value={setupData.email}
            placeholder="E-Mail"
        />
    </div>
    <div class="mb-6">
        <label
            class="block text-grey-darker text-sm font-bold mb-2"
            for="password"
        >
            Password
        </label>
        <input
            class="shadow appearance-none border border-red rounded w-full py-2 px-3 text-gray-800 mb-3"
            bind:value={setupData.password}
            id="password"
            type="password"
            placeholder="******************"
        />
        <!--        <p class="text-red text-xs italic">Please choose a password.</p>-->
    </div>
    <div class="flex items-center justify-between">
        <button
            class="bg-blue-700 hover:bg-blue-dark text-white font-bold py-2 px-4 rounded disabled:opacity-60"
            type="button"
            disabled={!inputValid}
            on:click={completeSetup}
        >
            Setup!
        </button>
    </div>
    {#if !inputValid}
        <p class="text-red-500 text-center w-fit mx-auto">
            {yupErrorMessage}
        </p>
    {/if}
</div>
