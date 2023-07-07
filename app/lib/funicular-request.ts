interface FunicularRequest extends RequestInit {
  url: URL;

  fetch(): Promise<Response>;

  path(input: string): FunicularRequest;
  select(input: string | string[]): FunicularRequest;

  single(): FunicularRequest;

  eq(key: string, value: string): FunicularRequest;
  ilike(key: string, value: string): FunicularRequest;
}

export function funicularRequest(): FunicularRequest {
  return {
    url: new URL("", Deno.env.get("SERVER") ?? "http://localhost:3000/"),
    fetch() {
      return fetch(this.url, this);
    },
    path(input) {
      this.url.pathname = input;
      return this;
    },
    select(input) {
      this.url.searchParams.set(
        "select",
        typeof input === "object" ? input.join(",") : input,
      );
      return this;
    },
    single() {
      return {
        headers: {
          Accept: "application/vnd.pgrst.object+json",
          ...this.headers,
        },
        ...this,
      };
    },
    eq(key, value) {
      this.url.searchParams.set(key, "eq." + value);
      return this;
    },
    ilike(key, value) {
      this.url.searchParams.set(key, "ilike." + value);
      return this;
    },
  };
}
