all:
	@echo "use 'make compile file-name', without .rs to compile the file"
	@echo "use 'make run file-name' to run the file"
	@echo "use 'make clean file-name' to clean the compilation archives"


# Variável para capturar o nome do arquivo
FILE := $(word 2, $(MAKECMDGOALS))

# Regra principal que compila o arquivo
compile: $(FILE)
	@echo "Compilação concluída!"

# Regra que compila o arquivo se ele existir
$(FILE):
	@if [ -f "src/$(FILE).rs" ]; then \
		rustc src/$(FILE).rs; \
	else \
		echo "Erro: Arquivo $(FILE) não encontrado!"; \
		exit 1; \
	fi

# Regra para rodar o programa compilado
run: $(FILE)
	@./$<

# Regra para limpar os arquivos gerados
clean: $(FILE)
	@rm -f $<

# Impede o Make de tentar criar um arquivo com o nome do argumento
%:
	@:
