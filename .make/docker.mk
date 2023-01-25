DOCKER_BUILD_PATH ?= docker
DUMP1090_IMAGE = $(PACKAGE_NAME)-dump1090
FEEDER_IMAGE   = $(PACKAGE_NAME)-feeder

.help-docker:
	@echo ""
	@echo "$(SMUL)$(BOLD)$(GREEN)Docker targets$(SGR0)"
	@echo "  $(BOLD)docker-build$(SGR0)   -- Run docker build to create new image"
	@echo "  $(BOLD)docker-run$(SGR0)     -- Run docker container as a daemon, binding port $(HOST_PORT):$(DOCKER_PORT)"
	@echo "  $(BOLD)docker-stop$(SGR0)    -- Run 'docker kill $${DOCKER_NAME}-run' to stop our docker after running"

docker-build:
	@echo "$(CYAN)Building $(DUMP1090_IMAGE) image...$(SGR0)"
	@DOCKER_BUILDKIT=1 docker build --build-arg PACKAGE_NAME=$(DUMP1090_IMAGE) --tag $(DUMP1090_IMAGE):latest docker/dump1090/
	
	@echo "$(CYAN)Building $(FEEDER_IMAGE) image...$(SGR0)"
	@DOCKER_BUILDKIT=1 docker build --build-arg PACKAGE_NAME=$(FEEDER_IMAGE) --tag $(FEEDER_IMAGE):latest .
