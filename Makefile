PHONY: run_ui

run_ui:
	cd apps/ui && trunk serve --proxy-backend=http://localhost:8081/tasks --open

run_server:
	# wip