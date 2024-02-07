#!/bin/bash


#############
# PORT SCAN #
#############
# Script destinado a ser um port scanner utilizando o switch case do Linux.


ip_address=0.0.0.0
range_ports_min=0
range_ports_max=80


while true; do

        clear
        cat << EOF
        Bem vindo ao PortScanner. POr favor, selecione a opcao desejada:

          1. Iniciar o Scan
          2. Editar IP/host para escanear [$ip_address] 
          3. Range de portas que serao escaneadas [$range_ports_min-$range_ports_max]
          4. Sair
EOF
        read -p "Selecione uma opcao [1-4] >"
        case "$REPLY" in

        1)
                echo "... Escaneando o host $ip_address ..."
                #Proxima versao ser[a recursiva para fins de testes.
                for porta in $(seq $range_ports_min $range_ports_max); do
                        if timeout .5 bash -c "</dev/tcp/$ip_address/$porta &>/dev/null"; then
                                echo "Porta $porta aberta"
                        else
                                echo "Porta $porta possivelmente fechada"
                        fi
                done
                echo "... Escaneamento concluido ..."
                read -p "Pressione ENTER p/ continuar"
                ;;

        2) 
                echo "Digite o novo IP: "
                read ip_address
                ;;
        3)
                echo "Digite a porta inicial: "
                read range_ports_min
                echo "Digite a porta final: "
                read range_ports_max
                ;;
        4)
                break
                ;;
        *)
                echo "Erro, opcao nao tratada. Tente outra."
                sleep 3
                ;;
        esac
done