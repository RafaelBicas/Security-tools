#!/bin/bash

#######################
#FERRAMENTA DE BACKUP #
#######################

# Ferramenta de exemplo para realizar backup de um arquivo
##########################################################

backup_path="/home/ellie/Pictures" #Qual o caminho para os arquivos que devem ser realizado os backups
dest="/home/ellie/Documents/" # Para onde estes arquivos devem ir 

backup_name="bp_hash" # nomenclatura que vir[a no in[icio do arquivo de backup
day=$(date +%D)
hostname=$(hostname -s)

backup_file="$backup_name-$hostname-$day.tgz"

echo "Iniciando o backup $backup_file. Aguarde alguns minutos."

tar -czf "$dest/$backup_file" $backup_path

#echo "Backup Finalizado"

#echo "MD5SUM P/ CONFERENCIA"
#md5sum $dest/$backup_file
#echo "--------------------"