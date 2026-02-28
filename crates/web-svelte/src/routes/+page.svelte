<script lang="ts">
	import 'bulma/css/bulma.css';
	import { onMount } from 'svelte';
	import { initialize, update } from '$lib/core';
	import view from '$lib/core';
	import {
		EventVariantReset,
		EventVariantIncrement,
		EventVariantDecrement
	} from 'shared_types/app';

	onMount(() => {
		initialize();
	});
</script>

<section class="box container has-text-centered m-5">
	<p class="is-size-5">{$view.count}</p>
	<div class="buttons section is-centered">
		<button class="button is-primary is-danger" onclick={() => update(new EventVariantReset())}>
			Reset
		</button>
		<button
			class="button is-primary is-success"
			onclick={() => update(new EventVariantIncrement())}
		>
			Increment
		</button>
		<button
			class="button is-primary is-warning"
			onclick={() => update(new EventVariantDecrement())}
		>
			Decrement
		</button>
	</div>
</section>
