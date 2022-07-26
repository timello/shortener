PKG_LIST=$(shell go list ./... | grep -v /vendor/)

all: build

test:
	@go test -race -short -v $(PKG_LIST)

clean:
	@go clean
	@rm -f bin/*

build:
	@env GOOS=linux GOARCH=amd64 go build -o bin/urlshortener ./cmd/urlshortener
	@zip -j bin/urlshortener.zip bin/urlshortener
