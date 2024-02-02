# Kanji Database

Esta es una base de datos MongoDB (no-SQL) para almacenar los datos de kanji-data. Por el momento solamente instala los kanjis, no los componentes.

# Requisitos
 - Docker
 - Python 3
 - python3-pip

# Despliegue

Para desplegar la base de datos tan solo es necesario tener instalado docker en su versión más reciente e instalar el siguiente comando.

```
docker compose up -d
```

Docker comenzará a descargar las imágenes y a montar el contenedor. Para detener el despliegue se puede ejecutar el siguiente comando.

```
docker compose down
```

# Inserción de los datos

Esta herramienta incluye un script de python para insertar los datos en la base de datos. En primer lugar, es necesario instalar las bibliotecas necesarias ejecutando lo siguiente.

```
pip install -r requirements.txt
```

Una vez instaladas las dependencias se pueden introducir los datos con el siguiente comando.

```
python3 kanji-upload.py
```
