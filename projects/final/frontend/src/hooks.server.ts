import { prepareStylesSSR } from '@svelteuidev/core';
import { sequence } from '@sveltejs/kit/hooks';

import { SvelteKitAuth } from '@auth/sveltekit';
import DiscordProvider from '@auth/core/providers/discord';
import { DISCORD_CLIENT_ID, DISCORD_CLIENT_SECRET } from "$env/static/private"


const scopes = ['identify', 'guilds', 'guilds.join', 'guilds.members.read', 'bot', 'email'].join(
	' '
);

export const handle = sequence(
	prepareStylesSSR,
	SvelteKitAuth({
		providers: [
			DiscordProvider({
				clientId: DISCORD_CLIENT_ID,
				clientSecret: DISCORD_CLIENT_SECRET,
				authorization: { params: { scope: scopes } }
			})
		]
	})
);
