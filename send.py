import asyncio
import json
import sys
from azure.eventhub.aio import EventHubProducerClient
from azure.eventhub import EventData

if len(sys.argv) != 3:
    print("Pass in connection string as first parameter and event hub name as second")
    sys.exit(1)


async def run():
    # Create a producer client to send messages to the event hub.
    # Specify a connection string to your event hubs namespace and
    # the event hub name.
    producer = EventHubProducerClient.from_connection_string(
        conn_str=sys.argv[1], eventhub_name=sys.argv[2])
    async with producer:
        # Create a batch.
        event_data_batch = await producer.create_batch()

        # Add events to the batch.
        event_data_batch.add(EventData(json.dumps({'key': '1'})))
        event_data_batch.add(EventData(json.dumps({'key': '2'})))
        event_data_batch.add(EventData(json.dumps({'key': '3'})))

        # Send the batch of events to the event hub.
        await producer.send_batch(event_data_batch)

loop = asyncio.get_event_loop()
loop.run_until_complete(run())
