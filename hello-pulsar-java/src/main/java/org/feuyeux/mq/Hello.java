package org.feuyeux.mq;

import java.util.concurrent.TimeUnit;
import org.apache.pulsar.client.api.Consumer;
import org.apache.pulsar.client.api.Producer;
import org.apache.pulsar.client.api.PulsarClient;
import org.apache.pulsar.client.api.PulsarClientException;

public class Hello {

  public static void main(String[] args) throws PulsarClientException, InterruptedException {
    PulsarClient client = PulsarClient.builder()
        .serviceUrl("pulsar://localhost:6650")
        //.serviceUrl("pulsar://localhost:6650,localhost:6651,localhost:6652")
        .build();

    Producer<byte[]> producer = client.newProducer()
        .topic("hello_topic")
        .create();
    for (int i = 0; i < 10; i++) {
      String msg = "Hello-Java-" + i;
      producer.sendAsync(msg.getBytes()).thenAccept(msgId -> {
        System.out.println("Message with ID " + msgId + " successfully sent");
      });
    }

    Consumer consumer = client.newConsumer()
        .topic("hello_topic")
        .subscriptionName("sub_java")
        .messageListener((c, msg) -> {
          try {
            String id = msg.getMessageId().toString();
            String data = new String(msg.getData());
            System.out.println("Message received: [" + id + "," + data + "]");
            c.acknowledge(msg);
          } catch (Exception e) {
            c.negativeAcknowledge(msg);
          }
        })
        .subscribe();

    TimeUnit.SECONDS.sleep(10);
    producer.close();
    consumer.close();
    client.close();
    System.out.println("Bye");
  }
}
