YEAR_MONTH := 2024-02
OUTPUT_DIR := output
OUTPUT_FILE := $(OUTPUT_DIR)/output_$(YEAR_MONTH).md
GZIP_FILE := $(OUTPUT_DIR)/$(YEAR_MONTH).md.gz

.PHONY: concat_and_compress

concat_and_compress:
	cat $(OUTPUT_DIR)/output_$(YEAR_MONTH)-*.md > $(OUTPUT_FILE)
	gzip -c $(OUTPUT_FILE) > $(GZIP_FILE)
	git add $(GZIP_FILE)
	git rm -f $(OUTPUT_DIR)/output_$(YEAR_MONTH)-*.md

view:
	jq -r 'group_by(.category)[] | {(.[0].category): [.[].url]}' config.json
