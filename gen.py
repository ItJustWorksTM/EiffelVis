from eiffellib.publishers import RabbitMQPublisher
from eiffellib.events import EiffelActivityTriggeredEvent

PUBLISHER = RabbitMQPublisher(host="127.0.0.1", exchange="amq.fanout", ssl=False,
                              port=5672)
PUBLISHER.start()

for _ in range(0, 10000):
    ACTIVITY_TRIGGERED = EiffelActivityTriggeredEvent()
    ACTIVITY_TRIGGERED.data.add("name", "Test activity")
    PUBLISHER.send_event(ACTIVITY_TRIGGERED, False)

input("press enter")