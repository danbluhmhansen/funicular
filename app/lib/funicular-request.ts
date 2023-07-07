interface FunicularRequest extends RequestInit {
  url: URL;

  fetch(): Promise<Response>;

  get(): FunicularRequest;
  post(body?: BodyInit): FunicularRequest;
  put(body?: BodyInit): FunicularRequest;
  delete(): FunicularRequest;

  single(): FunicularRequest;

  path(input: string | string[]): FunicularRequest;

  select(input: string | string[]): FunicularRequest;

  eq(key: string, value: string): FunicularRequest;
  ilike(key: string, value: string): FunicularRequest;
}

export function funicularRequest(): FunicularRequest {
  return {
    url: new URL("", Deno.env.get("SERVER") ?? "http://localhost:3000/"),

    fetch() {
      return fetch(this.url, this);
    },

    get() {
      return {
        method: "GET",
        body: undefined,
        ...this,
      };
    },
    post(body) {
      return {
        method: "POST",
        body,
        ...this,
      };
    },
    put(body) {
      return {
        method: "PUT",
        body,
        ...this,
      };
    },
    delete() {
      return {
        method: "DELETE",
        body: undefined,
        ...this,
      };
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

    path(input) {
      this.url.pathname = typeof input === "object" ? input.join("/") : input;
      return this;
    },

    select(input) {
      this.url.searchParams.set(
        "select",
        typeof input === "object" ? input.join(",") : input,
      );
      return this;
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
