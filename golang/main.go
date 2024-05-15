package main

import (
	"context"
	"net/http"
	"os"
	"os/signal"
	"syscall"

	"github.com/gin-gonic/gin"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

func main() {
	zerolog.TimeFieldFormat = zerolog.TimeFormatUnix
	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stderr})

	err := run()
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to run")
	}
}

func run() error {
	r := gin.Default()
	srv := &http.Server{
		Addr:    ":8080",
		Handler: r,
	}

	// Handle SIGTERM signal to gracefully shutdown the server
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGTERM)
	go func() {
		<-quit
		if err := srv.Shutdown(context.Background()); err != nil {
			log.Fatal().Err(err).Msg("Server shutdown failed")
		}
	}()

	r.GET("/health", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "Hello, World!",
		})
	})

	return srv.ListenAndServe()
}
