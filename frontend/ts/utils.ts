import mustache from "https://cdnjs.cloudflare.com/ajax/libs/mustache.js/4.2.0/mustache.min.js"

import { User } from "./datatypes"

export function pageLoad() {
	populateUsers()
}

export function populateUsers() {
	const users: User[] = [{ id: 0, username: "sean", created_at: new Date() }];

	const template = document.getElementById('template');
	if (!template) { return; }
	const rendered = mustache.render(template.innerHTML, { name: 'Luke' });
	const target = document.getElementById('target');
	if (!target) { return; }
	target.innerHTML = rendered;
}
