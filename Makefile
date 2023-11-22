test : 
	cargo test 

build :
	cargo build

sample :
	@cd examples/sample01; \
	terraform init ; \
	terraform plan -out sample01.out ; \
	terraform show -json sample01.out > sample01.json ; 