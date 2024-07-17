function basicAuth(username: string, password: string) {
  return "Basic " + btoa(`${username}:${password}`);
}

export interface Product {
  productId: string;
  type: string;
  description: string;
  shortDescription: string;
  area: string;
  address: string;
  floor: string;
  sqrMtrs: string;
  reserved: string;
  numberOfReservations: string;
  queueNumber: string;
  rent: string;
}

export async function getVacant(): Promise<Product[]> {
  const email = process.env.EMAIL;
  const password = process.env.PASSWORD;

  if (!email) throw new Error("EMAIL not set");
  if (!password) throw new Error("PASSWORD not set");

  const res = await fetch(
    "https://diremoapi.afbostader.se/redimo/rest/vacantproducts?lang=sv_SE&type=1",
    {
      headers: {
        authorization: basicAuth(email, password),
        // "user-agent":
          // "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0",
      },
    },
  ).then((res) => res.json());

  return res.product;
}
