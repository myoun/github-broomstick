<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'

    type Repository = {
        name: string,
        description: string | undefined,
        checked: boolean,
    }

    let token: string;

    let repositories: Repository[] = [];

    async function search() {
        const field = document.getElementById("token-field") as HTMLInputElement;
        const raw_token = field.value;

        if (raw_token.length > 0) {
            const hidden_token = "●".repeat(raw_token.length);
            field.value = hidden_token;
            field.disabled = true;
            token = raw_token;

            const repos: any[] = await invoke("search_repository", { token })
            repositories = repos.map((r) => {
                const repository: Repository = {
                    name: r.full_name,
                    description: r.description,
                    checked: false,
                };
                return repository;
            })
        }
    }

    async function clearToken() {
        const field = document.getElementById("token-field") as HTMLInputElement;

        if (field.value.length > 0) {
            field.value = "";
            field.disabled = false;
            repositories = [];
        }
    }

    function remove() {
        const checked = repositories.filter((r) => r.checked);
        invoke("delete_repositories", { token, repositories: checked });
        clearToken();
    }
</script>

<input id="token-field" placeholder="Enter a github api token..." style="width: 240px;">
<button id="search-buttonn" on:click={search}>Search</button>
<button id="clear-button" on:click={clearToken}>Clear</button>

<ul>
    {#each repositories as repo}
    <li>
        <input type="checkbox" bind:checked={repo.checked} />
        <h3>{repo.name}</h3>
        {#if repo.description != undefined}
            <p>{repo.description}</p>
        {/if}
    </li>
    {/each}
</ul>

<br/>

<span>Selected: {repositories.filter((r) => r.checked).length}/{repositories.length}</span>
<button style="color: red;" on:click={remove}>Delete Repositories</button>