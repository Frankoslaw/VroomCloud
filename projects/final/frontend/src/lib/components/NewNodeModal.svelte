<script lang="ts">
    import { Title, Group, Button, Modal, TextInput } from '@svelteuidev/core';

    export let opened = false;

    function close_modal() {
        opened = false;
    }

    export let nodes: any[] = []

    let new_node_name: string = "";
    let new_node_ip: string = "";

    function add_node() {
        nodes = [...nodes, {
            node_name: new_node_name,
            ip: new_node_ip,
            state: "Running",
            version: 0.3
        }]
        console.log(nodes);
        opened = false;
    }
</script>

<Modal {opened} on:close={close_modal}>
    <Title class="mb-5">Add existing node:</Title>
    <TextInput
        class="mb-2"
        placeholder="Node name"
        label="Node name:"
        bind:value={new_node_name}
    />
    <TextInput
        class="mb-10"
        placeholder="Node IP"
        label="Node IP:"
        bind:value={new_node_ip}
    />
    <Group position="right">
        <Button color="green" class="p-5" on:click={add_node}>Add!</Button>
    </Group>
</Modal>
