export const API_URL =
	typeof window === 'undefined'
		? (import.meta.env.VITE_API_URL ?? 'http://localhost:8000')
		: (import.meta.env.VITE_API_URL ?? 'http://localhost:8000');

export interface Address {
	street: string;
	city: string;
	postal_code: string;
}

export interface QueuePosition {
	position: number | null;
	total_in_queue: number;
}

export type PropertyType = 'Apartment' | 'Dorm';

export interface Property {
	id: number;
	property_type: PropertyType;
	area: string;
	description: string;
	short_description: string;
	address: Address;
	floor: number;
	size_sqm: number;
	reserved: boolean;
	queue_position: QueuePosition;
	rent: number;
	reserve_from: string;
	reserve_until: string;
	move_in: string;
}

export interface PropertyDetail extends Property {
	facing: string;
}

export interface Picture {
	alt: string | null;
	url: string;
}

export interface AreaDetail {
	pictures: Picture[];
}

export interface EmailPassword {
	email: string;
	password: string;
}

export type LoginResponse = UserDetails | 'invalid-credentials';

export interface UserDetails {
	first_name: string;
	last_name: string;
}

export async function listVacancies(): Promise<Property[]> {
	const res = await fetch(`${API_URL}/vacancies`, {
		credentials: 'include'
	});
	return res.json();
}

export async function getVacancy(id: number): Promise<PropertyDetail> {
	const res = await fetch(`${API_URL}/vacancies/${encodeURIComponent(id)}`, {
		credentials: 'include'
	});
	return res.json();
}

export async function getArea(areaName: string): Promise<AreaDetail> {
	const res = await fetch(`${API_URL}/areas/${encodeURIComponent(areaName)}`, {
		credentials: 'include'
	});
	return res.json();
}

export async function getUser(): Promise<UserDetails | 'unauthenticated'> {
	const res = await fetch(`${API_URL}/user`, {
		credentials: 'include'
	});
	if (res.ok) return res.json();
	return 'unauthenticated';
}

export async function login(details: EmailPassword): Promise<LoginResponse> {
	const res = await fetch(`${API_URL}/login`, {
		method: 'POST',
		headers: { 'content-type': 'application/json' },
		body: JSON.stringify(details),
		credentials: 'include'
	});
	if (res.ok) return res.json();
	if (res.status === 403) return 'invalid-credentials';
	throw new Error(`unexpected response: ${await res.text()}`);
}

export async function logout(): Promise<void> {
	await fetch(`${API_URL}/logout`, { credentials: 'include' });
}

export interface Place {
	lat: number;
	lon: number;
}

export async function geocode(address: Address): Promise<Place[]> {
	const params = new URLSearchParams({
		street: address.street,
		postalcode: address.postal_code,
		city: address.city
	});
	const res = await fetch(`${API_URL}/geocode?${params}`);
	return res.json();
}
