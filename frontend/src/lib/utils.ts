export function formatPostalCode(postalCode?: string) {
  const match = postalCode?.match(/^(\d{3})(\d{2})$/);
  if (!match) return postalCode;
  return `${match[1]}\xa0${match[2]}`;
}
