export async function http<T>(url: string, req?: RequestInit): Promise<T> {
	const resp = await fetch(url, req);
	const data = await resp.json();
	return data;
}

export function castFieldToDate<T>(arr: T[], fields: (keyof T)[]) {
	let j;
	for (let i = 0; i < arr.length; i++) {
		for (j = 0; j < fields.length; j++) {
			(arr[i][fields[j]] as any) = new Date((arr[i][fields[j]] as any));
		}
	}
}

export function castNestedFieldToDate<T, G>(
	arr: T[], fields_nested: (keyof T)[],
	outer_cast_fields: (keyof T)[],
	inner_cast_fields: (keyof G)[]
) {
	let j;
	for (let i = 0; i < arr.length; i++) {
		for (j = 0; j < outer_cast_fields.length; j++) {
			(arr[i][outer_cast_fields[j]] as any) = new Date((arr[i][outer_cast_fields[j]] as any));
		}
		for (j = 0; j < fields_nested.length; j++) {
			castFieldToDate((arr[i][fields_nested[j]] as any), inner_cast_fields);
		}
	}
}

export function getInpVal(id: string): string {
	const elem = document.getElementById(id) as HTMLInputElement;
	return elem.value;
}

export function getInpCheck(id: string): boolean {
	const elem = document.getElementById(id) as HTMLInputElement;
	return elem.checked;
}

export function getSelectVal(id: string): string {
	const elem = document.getElementById(id) as HTMLSelectElement;
	return elem.value;
}
