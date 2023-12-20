<script>
	// @ts-nocheck

	import { Group, Title, Image, Text, Flex } from '@svelteuidev/core';

    import { page } from '$app/stores';
    let { user } = $page.data.session;
</script>

<Flex justify="space-between" class="p-5">
	<Group>
        <Title><b>VroomCloud :D</b></Title>
    </Group>
    <Group class="mb-3">
        <Image src={user?.image} alt="" width={38} height={38} radius="xl" />
        <Text size="xl" weight={'bold'} color="green">{user?.name}</Text>
    </Group>
</Flex>
