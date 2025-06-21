import { createResource, Suspense, type Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

// Import transport from client, and generated server type
import { http, build_client } from "@qubit-rs/client";
import type { QubitServer } from "../bindings";

const api = build_client<QubitServer>(http("http://localhost:8080/rpc")); // TODO: load from env and set from RUST if possible at rs runtime

const App: Component = () => {
  const [message] = createResource(api.hello_world.query);

  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <img src={logo} class={styles.logo} alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          class={styles.link}
          href="https://github.com/solidjs/solid"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn Solid
        </a>
        <Suspense fallback={"Loading..."}>
          <p>Backend says '<code>{message()}</code>' from RPC ✨</p>
        </Suspense>
      </header>
    </div>
  );
};

export default App;
