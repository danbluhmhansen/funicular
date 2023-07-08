export default function funRequest() {
  return new FunRequest(
    new URL(
      "",
      Deno.env.get("SERVER") ?? "http://localhost:3000/",
    ),
    {},
  );
}

class FunRequest {
  constructor(url: URL, init: RequestInit) {
    this.url = url;
    this.init = init;
  }

  private init: RequestInit;
  private url: URL;

  fetch() {
    return fetch(this.url, this.init);
  }

  get() {
    this.init.method = "GET";
    this.init.body = undefined;
    return this;
  }
  post(body?: BodyInit) {
    this.init.method = "POST";
    this.init.body = body;
    return this;
  }
  put(body?: BodyInit) {
    this.init.method = "PUT";
    this.init.body = body;
    return this;
  }
  delete() {
    this.init.method = "DELETE";
    this.init.body = undefined;
    return this;
  }

  single() {
    this.init.headers = {
      ...this.init.headers,
      Accept: "application/vnd.pgrst.object+json",
    };
    return this;
  }

  path(input: string | string[]) {
    this.url.pathname = typeof input === "object" ? input.join("/") : input;
    return this;
  }

  select(input: string | string[]) {
    this.url.searchParams.set(
      "select",
      typeof input === "object" ? input.join(",") : input,
    );
    return this;
  }

  eq(key: string, value: string) {
    this.url.searchParams.set(key, "eq." + value);
    return this;
  }
  ilike(key: string, value: string) {
    this.url.searchParams.set(key, "ilike." + value);
    return this;
  }
}
