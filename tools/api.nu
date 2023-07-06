#!/usr/bin/env nu

def main [--url (-u) = "http://localhost:3000/"] {
  let select = ([
    "*",
    "actor_kind!inner(game!inner(),skill(name))",
    "actor_trait(trait(name))",
    "actor_gear(gear(name))",
  ] | str join ",")
  let query = ([
    "actor_kind.game.name=eq.foo",
    "name=ilike.jaudenn runecleaver",
  ] | str join "&")
  let url = $"($url)actor?select=($select)($query)"
  http get --raw $url
}
