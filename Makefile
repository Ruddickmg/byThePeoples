default: postgres auth-service

postgres:
	docker start postgres

auth-service:
	cargo run --manifest-path=./services/auth

test-auth-service:
	corgo test --manifest-path=./services/auth