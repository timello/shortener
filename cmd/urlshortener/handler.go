package main

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/aws/aws-lambda-go/events"
)

type Handler struct {
}

type RequestBody struct {
	URL string `json:"url"`
}

func (h *Handler) URLShortenerHandler(ctx context.Context, event events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {

	fmt.Printf("event: %+v\n", event.Body)

	body := &RequestBody{}

	err := json.Unmarshal([]byte(event.Body), body)
	if err != nil {
		return events.APIGatewayProxyResponse{
			StatusCode: 500,
		}, err

	}

	return events.APIGatewayProxyResponse{
		StatusCode: 200,
		Body:       body.URL,
	}, nil
}
