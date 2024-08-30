.PHONY: up
up:
	docker compose up -d

.PHONY: build
build:
	docker compose build --no-cache

.PHONY: down
down:
	docker compose down -v

.PHONY: clean
clean:
	docker system prune -a --volumes -f && docker builder prune -a -f

.PHONY: end
end: down clean

.PHONY: again
again: down clean build up
