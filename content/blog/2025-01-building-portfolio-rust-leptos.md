---
slug: building-portfolio-rust-leptos
date: 2025-01-15
tags: [rust, leptos, web, portfolio]
title_en: Building a Portfolio with Rust and Leptos
title_fr: Construire un Portfolio avec Rust et Leptos
summary_en: Why I chose Rust and Leptos for my personal portfolio, and the journey of building a performant, type-safe web application.
summary_fr: Pourquoi j'ai choisi Rust et Leptos pour mon portfolio personnel, et le parcours de construction d'une application web performante et type-safe.
---

<!-- EN -->
# Building a Portfolio with Rust and Leptos

When I decided to rebuild my portfolio, I wanted something different. Not another React app, not another static site generator. I wanted to explore **Rust for the web**.

## Why Rust?

Rust brings several advantages to web development:

- **Type safety** - Catch errors at compile time, not in production
- **Performance** - Near-native speed with WebAssembly
- **Memory safety** - No null pointer exceptions, no data races

```rust
// A simple Leptos component
#[component]
fn HelloWorld() -> impl IntoView {
    let name = "World";
    view! {
        <h1>"Hello, " {name} "!"</h1>
    }
}
```

## The Leptos Framework

Leptos is a full-stack Rust framework that compiles to WebAssembly for the frontend and runs natively on the server. It offers:

1. **Fine-grained reactivity** - Only updates what changes
2. **SSR + Hydration** - Fast initial loads, interactive afterwards
3. **Type-safe routing** - No runtime route errors

## Challenges Along the Way

Building with Rust isn't always smooth. The ecosystem is younger than JavaScript's, and compile times can be slow. But the payoff in reliability and performance makes it worthwhile.

## What's Next?

This blog will document my journey building cloud infrastructure, exploring DevOps practices, and pushing the boundaries of what's possible with Rust on the web.

<!-- FR -->
# Construire un Portfolio avec Rust et Leptos

Quand j'ai decide de reconstruire mon portfolio, je voulais quelque chose de different. Pas une autre application React, pas un autre generateur de site statique. Je voulais explorer **Rust pour le web**.

## Pourquoi Rust ?

Rust apporte plusieurs avantages au developpement web :

- **Securite des types** - Detecter les erreurs a la compilation, pas en production
- **Performance** - Vitesse quasi-native avec WebAssembly
- **Securite memoire** - Pas d'exceptions de pointeur null, pas de data races

```rust
// Un composant Leptos simple
#[component]
fn HelloWorld() -> impl IntoView {
    let name = "World";
    view! {
        <h1>"Bonjour, " {name} "!"</h1>
    }
}
```

## Le Framework Leptos

Leptos est un framework Rust full-stack qui compile en WebAssembly pour le frontend et s'execute nativement sur le serveur. Il offre :

1. **Reactivite fine** - Ne met a jour que ce qui change
2. **SSR + Hydratation** - Chargements initiaux rapides, interactif ensuite
3. **Routage type-safe** - Pas d'erreurs de route a l'execution

## Les Defis en Cours de Route

Construire avec Rust n'est pas toujours facile. L'ecosysteme est plus jeune que celui de JavaScript, et les temps de compilation peuvent etre longs. Mais le gain en fiabilite et performance en vaut la peine.

## Et Ensuite ?

Ce blog documentera mon parcours de construction d'infrastructure cloud, d'exploration des pratiques DevOps, et de repousser les limites de ce qui est possible avec Rust sur le web.
