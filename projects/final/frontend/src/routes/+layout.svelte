<script>
    // @ts-nocheck

    import "../app.css";
    import {
      colorScheme,
      SvelteUIProvider,
      AppShell,
      Affix,
      Button,
      Header
    } from "@svelteuidev/core";
    import { Moon } from "radix-icons-svelte"; // radix-icons-breaks typechecking
    import Navbar from "$lib/components/Navbar.svelte";

    colorScheme.update(() => "dark");
    function toggleTheme() {
      colorScheme.update((v) => (v === "light" ? "dark" : "light"));
    }
  </script>

  <SvelteUIProvider
    withNormalizeCSS
    withGlobalStyles
    themeObserver={$colorScheme}
  >
    <AppShell>
      <Header slot="header" class="p-1">
        <Navbar />
      </Header>

      <main class="p-1">
        <slot />

        <Affix position={{ bottom: 20, right: 20 }}>
          <Button on:click={toggleTheme} radius="xl" color="green" size="md">
            <Moon />
          </Button>
        </Affix>
      </main>
    </AppShell>
  </SvelteUIProvider>
