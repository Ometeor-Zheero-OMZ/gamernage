# DEV
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

# TEST
.PHONY: test-up
test-up: 
	docker compose -f docker-compose_dev.yml up -d

.PHONY: test-build
test-build:
	docker compose -f docker-compose_dev.yml build --no-cache

.PHONY: test-down
test-down: 
	docker compose -f docker-compose_dev.yml down -v

.PHONY: test-end
test-end: test-down clean

.PHONY: test-again
test-again: test-down clean test-build test-up