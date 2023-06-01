import os
import shutil

# Указываем путь к папке, в которой находятся файлы
folder_path = r'C:\Users\Dmitrii\Desktop\enron2'

# Получаем список файлов в папке
files = os.listdir(folder_path)

# Открываем файл для записи
with open(r'C:\Users\Dmitrii\Desktop\myy\merged.txt', 'wb') as outfile:
    # Проходим по всем файлам в папке
    for file in files:
        # Открываем каждый файл для чтения
        with open(os.path.join(folder_path, file), 'rb') as infile:
            # Копируем содержимое файла в выходной файл
            shutil.copyfileobj(infile, outfile)
        os.remove(os.path.join(folder_path, file))

