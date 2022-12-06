import App from './App.svelte';
import { EiffelVisConnection } from './eiffelvis';

const backend_url: string = process.env.EIFFELVIS_URL.startsWith("@origin")
	? `${window.location.host}${window.location.pathname
	}${process.env.EIFFELVIS_URL.replace("@origin", "")}`
	: process.env.EIFFELVIS_URL.startsWith("@hostname")
		? `${window.location.hostname}${process.env.EIFFELVIS_URL.replace(
			"@hostname",
			""
		)}`
		: process.env.EIFFELVIS_URL;
const backend_has_ssl = JSON.parse(process.env.EIFFELVIS_SSL);

const connection: EiffelVisConnection = new EiffelVisConnection(
	backend_url, backend_has_ssl
)

const app = new App({
	target: document.body,
	props: {
		connection
	}
});

export default app;
