<script>
    import { Card, Title, Space, Group, Badge, Button } from '@svelteuidev/core';
    import { Pencil2 } from "radix-icons-svelte";

    export let node_name = "";
    export let ip = "0.0.0.0";
    export let state = "Running";
    export let version = "v0.1";
</script>

<Card shadow="md">
    <Group position="apart">
        <Title order={2}>{node_name}</Title>
        <Group>
            IP:
            <Badge size="xl" radius="sm" color="cyan">
                {ip}
            </Badge>
            <Space />
            Status:
            <Badge size="xl" radius="sm" color="green">
                {state}
            </Badge>
            Version:
            <Badge size="xl" radius="sm" color="grape">
                {version}
            </Badge>
            <Space w="xl" />
            <Button color="yellow">
                <Pencil2 />
            </Button>
        </Group>
    </Group>
</Card>
