import { CoreFFI } from 'shared';
import init_core from 'shared/shared';
import { writable } from 'svelte/store';
import type { Event } from 'shared_types/app';
import { EffectVariantRender, Request, ViewModel } from 'shared_types/app';
import { BincodeDeserializer, BincodeSerializer } from 'shared_types/bincode';

// Type-safe helper for Bincode
const fromBytes = <T>(bytes: Uint8Array, Type: { deserialize: (d: BincodeDeserializer) => T }): T =>
	Type.deserialize(new BincodeDeserializer(bytes));

let core: CoreFFI;
const { subscribe, set } = writable(new ViewModel(''));

export async function initialize() {
	if (core) return;
	await init_core();
	core = new CoreFFI();
	set(fromBytes(core.view(), ViewModel));
}

export function update(event: Event) {
	const serializer = new BincodeSerializer();
	event.serialize(serializer);

	const effectBytes = core.update(serializer.getBytes());
	const deserializer = new BincodeDeserializer(effectBytes);

	// Inline the loop to avoid temporary array allocations
	const len = deserializer.deserializeLen();
	for (let i = 0; i < len; i++) {
		const { effect } = Request.deserialize(deserializer);

		// Use 'instanceof' for clean, type-safe branching
		if (effect instanceof EffectVariantRender) {
			set(fromBytes(core.view(), ViewModel));
		}
	}
}

export default { subscribe };