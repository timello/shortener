package main

import (
	"context"
	"fmt"
)

type Handler struct {
}

func (h *Handler) URLShortenerHandler(ctx context.Context, request interface{}) error {

	fmt.Printf("request: %+v\n", request)
	return nil
}
