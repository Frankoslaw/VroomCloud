<script>
    import { Stack, Card, Title, Group, Button, Modal, TextInput } from '@svelteuidev/core';
    import { Plus } from "radix-icons-svelte";

    import NodeCard from '$lib/components/NodeCard.svelte';
    import NewNodeModal from '$lib/components/NewNodeModal.svelte';

    let new_node_opened = false;
    let nodes = [
        {
            node_name: "Aws Frankfurt 1",
            ip: "127.0.0.1",
            state: "Running",
            version: 0.2
        }
    ]
</script>


<Stack>
    <Card shadow="md">
        <Title class="mb-10">Nodes</Title>

        <Stack>
            {#each nodes as node}
                <NodeCard
                    node_name={node.node_name}
                    ip={node.ip}
                    state={node.state}
                    version={"v" + node.version}/>
            {/each}
            <Group position="right" class="m-5">
                <Button color="green" on:click={() => (new_node_opened = true)}>
                    <Plus/>
                </Button>
            </Group>
        </Stack>
    </Card>
    <Card>
        <Title>ETL's</Title>
    </Card>
</Stack>

<NewNodeModal bind:opened={new_node_opened} bind:nodes />
