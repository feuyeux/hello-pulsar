package main

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/apache/pulsar-client-go/pulsar"
)

func main() {
	client, err := pulsar.NewClient(pulsar.ClientOptions{
		URL: "pulsar://localhost:6650",
	})

	if err != nil {
		log.Fatal(err)
	}

	defer client.Close()

	go func() {
		producer, err := client.CreateProducer(pulsar.ProducerOptions{
			Topic: "hello_topic",
		})
		if err != nil {
			log.Fatal(err)
		}

		defer producer.Close()

		ctx := context.Background()

		for i := 0; i < 10; i++ {
			if msgId, err := producer.Send(ctx, &pulsar.ProducerMessage{
				Payload: []byte(fmt.Sprintf("Hello-Go-%d", i)),
			}); err != nil {
				log.Fatal(err)
			} else {
				log.Println("Message sent: ", msgId)
			}
		}
	}()

	go func() {
		consumer, err := client.Subscribe(pulsar.ConsumerOptions{
			Topic:            "hello_topic",
			SubscriptionName: "sub_go",
			Type:             pulsar.Shared,
		})
		if err != nil {
			log.Fatal(err)
		}
		defer consumer.Close()

		for i := 0; i < 10; i++ {
			msg, err := consumer.Receive(context.Background())
			if err != nil {
				log.Fatal(err)
			}

			fmt.Printf("Message received: [%s,'%s']\n", msg.ID().String(), string(msg.Payload()))
			ackErr := consumer.Ack(msg)
			if ackErr != nil {
				return
			}
		}

		if err := consumer.Unsubscribe(); err != nil {
			log.Fatal(err)
		}
	}()

	time.Sleep(10 * time.Second)

	fmt.Printf("Bye")
}
