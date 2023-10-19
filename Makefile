
.PHONY: frontend
frontend:
	@echo "Building frontend..."
	@cd frontend && npm start nodemon server
