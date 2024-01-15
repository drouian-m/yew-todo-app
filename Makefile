.PHONY: reload_db
reload_db:
	cd apps/server && rm database.db && diesel setup && diesel migration run

.PHONY: run_ui
run_ui:
	cd apps/ui && trunk serve --proxy-backend=http://localhost:8081/tasks --open

.PHONY: run_server
run_server:
	cd apps/server && cargo run